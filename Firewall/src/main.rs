use dashmap::DashMap;
use tokio::runtime::Builder;
use std::collections::HashMap;
use std::collections::HashSet;
use std::os::unix::net::UnixDatagram;
use std::process::Command;
use std::sync::Arc;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::{fs, thread};
use std::{net::UdpSocket, path::Path};
use tokio::{
    io::BufReader,
    time::{self, Duration},
};
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt},
    net::{UnixListener, UnixStream},
};
use warp::Filter;
use serde::{Deserialize, Serialize};

const SERVER_WEB: &str = "http://localhost/api";
const SERVER_IP: &str = "192.168.1.1";
const CHECK_INTERVAL: Duration = Duration::from_secs(5);

#[derive(Debug)]
enum HostapdEvent {
    Connected(String),    // MAC
    Disconnected(String), // MAC
    Other(String),        // Texto completo
}

type MacIpMap = Arc<DashMap<String, String>>; // MAC -> IP
type AllowedIps = Arc<DashMap<String, ()>>;

/// ⚠️ Solo ejecutar una vez al inicio
pub fn setup_global_blocking() {
    // Limpia todo
    let _ = Command::new("iptables").args(["-F"]).status();
    let _ = Command::new("iptables").args(["-t", "nat", "-F"]).status();

    // Permite loopback
    let _ = Command::new("iptables")
        .args(["-A", "INPUT", "-i", "lo", "-j", "ACCEPT"])
        .status();

    // Aceptar tráfico hacia/desde el servidor si querés permitir el dashboard
    let _ = Command::new("iptables")
        .args(["-A", "FORWARD", "-d", SERVER_IP, "-j", "ACCEPT"])
        .status();
    let _ = Command::new("iptables")
        .args(["-A", "FORWARD", "-s", SERVER_IP, "-j", "ACCEPT"])
        .status();

    // BLOQUEAR TODO desde wlan0 (clientes)
    let _ = Command::new("iptables")
        .args(["-A", "FORWARD", "-i", "wlan0", "-j", "DROP"])
        .status();

    // Habilitar NAT para salida a Internet
    let _ = Command::new("iptables")
        .args([
            "-t",
            "nat",
            "-A",
            "POSTROUTING",
            "-o",
            "eth0",
            "-j",
            "MASQUERADE",
        ])
        .status();

    println!("[✓] Reglas aplicadas: bloqueo por defecto con NAT.");
}

pub fn allow_ip(ip: &str, allowed_ips: &AllowedIps) {
    if allowed_ips.contains_key(ip) {
        println!("[*] IP {} ya estaba desbloqueada", ip);
        return;
    }

    let _ = Command::new("iptables")
        .args(["-I", "FORWARD", "1", "-s", ip, "-j", "ACCEPT"])
        .status();
    let _ = Command::new("iptables")
        .args(["-I", "FORWARD", "1", "-d", ip, "-j", "ACCEPT"])
        .status();

    allowed_ips.insert(ip.to_string(), ());
    println!("[+] IP desbloqueada: {} (acceso total)", ip);
}

pub fn block_ip(ip: &str, allowed_ips: &AllowedIps) {
    if !allowed_ips.contains_key(ip) {
        println!("[*] IP {} ya estaba bloqueada", ip);
        return;
    }

    let _ = Command::new("iptables")
        .args(["-D", "FORWARD", "-s", ip, "-j", "ACCEPT"])
        .status();
    let _ = Command::new("iptables")
        .args(["-D", "FORWARD", "-d", ip, "-j", "ACCEPT"])
        .status();

    allowed_ips.remove(ip);
    println!("[-] IP bloqueada nuevamente: {}", ip);
}

// fn get_private_ip() -> String {
//     let socket = UdpSocket::bind("0.0.0.0:0").expect("No se pudo crear el socket");
//     socket
//         .connect("8.8.8.8:80")
//         .expect("No se pudo conectar al socket");
//     if let Ok(local_addr) = socket.local_addr() {
//         return local_addr.ip().to_string();
//     }
//     "0.0.0.0".to_string()
// }

fn parse_leases() -> std::io::Result<HashMap<String, String>> {
    let content = std::fs::read_to_string("/var/lib/misc/dnsmasq.leases")?;
    let mut map = HashMap::new();

    for line in content.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            let mac = parts[1].to_string();
            let ip = parts[2].to_string();
            map.insert(mac, ip);
        }
    }

    Ok(map)
}

fn hostapd_socket(tx: Sender<HostapdEvent>, mac_ip_map: MacIpMap) -> std::io::Result<()> {
    const CLIENT_PATH: &str = "/tmp/nethound.sock";
    const HOSTAPD_SOCKET: &str = "/var/run/hostapd/wlan0";

    if Path::new(CLIENT_PATH).exists() {
        fs::remove_file(CLIENT_PATH)?;
    }

    let sock = UnixDatagram::bind(CLIENT_PATH)?;
    sock.connect(HOSTAPD_SOCKET)?;
    sock.send(b"ATTACH")?;

    let mut buf = [0u8; 4096];
    println!("[*] Escuchando eventos de hostapd...");

    loop {
        let size = sock.recv(&mut buf)?;
        let msg = String::from_utf8_lossy(&buf[..size]);

        if msg.contains("AP-STA-CONNECTED") {
            if let Some(mac) = msg.split_whitespace().last() {
                // Buscá IP en leases y guardá en mapa
                if let Ok(leases) = parse_leases() {
                    if let Some(ip) = leases.get(mac) {
                        mac_ip_map.insert(mac.to_string(), ip.to_string());
                        println!("[+] Guardado MAC {} con IP {}", mac, ip);
                    }
                }
                tx.send(HostapdEvent::Connected(mac.to_string())).ok();
            }
        } else if msg.contains("AP-STA-DISCONNECTED") {
            if let Some(mac) = msg.split_whitespace().last() {
                mac_ip_map.remove(mac);
                println!("[-] Eliminado MAC {} del mapa", mac);
                tx.send(HostapdEvent::Disconnected(mac.to_string())).ok();
            }
        } else {
            tx.send(HostapdEvent::Other(msg.to_string())).ok();
        }
    }
}

async fn backend_http(ip: String, mac: String) -> std::io::Result<bool> {
    let url = format!("{}/userip?mac={}&ip={}", SERVER_WEB, mac, ip);

    println!("Enviando solicitud al backend: {}", url);
    if let Ok(response) = reqwest::get(&url).await {
        if let Ok(body) = response.text().await {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
                println!("Respuesta del backend: {:?}", json);
                if let Some(authorized) = json.get("authorized").and_then(|v| v.as_bool()) {
                    return Ok(authorized);
                }
            }
        }
    }

    Ok(false)
}

async fn run_http_server(allowed_ips: AllowedIps) -> Result<(), warp::Error> {
    #[derive(serde::Deserialize)]
    struct ReloadRequest {
        ips: Vec<String>,
    }

    let allowed_ips_filter = warp::any().map(move || allowed_ips.clone());

    let reload = warp::post()
        .and(warp::path("reload"))
        .and(warp::body::json())
        .and(allowed_ips_filter)
        .and_then(|body: ReloadRequest, allowed_ips: AllowedIps| async move {
            let new_ips: HashSet<_> = body.ips.into_iter().collect();

            print!("[*] Recibiendo nueva lista de IPs: {:?}\n", new_ips);

            // Permitir nuevas IPs
            for ip in &new_ips {
                if !allowed_ips.contains_key(ip) {
                    allow_ip(ip, &allowed_ips);
                }
            }

            // Bloquear IPs que ya no están
            let current_ips: Vec<String> = allowed_ips.iter().map(|e| e.key().clone()).collect();
            for ip in current_ips {
                if !new_ips.contains(&ip) {
                    block_ip(&ip, &allowed_ips);
                }
            }

            Ok::<_, warp::Rejection>(warp::reply::json(&serde_json::json!({"status": "ok"})))
        });

    println!("Servidor HTTP escuchando en http://0.0.0.0:3030");
    warp::serve(reload).run(([0, 0, 0, 0], 3030)).await;

    Ok(())
}

fn main() -> std::io::Result<()> {
    setup_global_blocking();

    let (tx, rx) = mpsc::channel();
    let mac_ip_map: MacIpMap = Arc::new(DashMap::new());
    let allowed_ips: AllowedIps = Arc::new(DashMap::new());

     // Lanzar servidor HTTP en hilo aparte con runtime Tokio
    {
        let allowed_ips_clone = allowed_ips.clone();
        std::thread::spawn(move || {
            let rt = Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap();
            rt.block_on(run_http_server(allowed_ips_clone))
                .unwrap_or_else(|e| eprintln!("Error en servidor HTTP: {}", e));
        });
    }

    // 🧵 Hilo para hostapd
    {
        let tx_clone = tx.clone();
        let map_clone = mac_ip_map.clone();
        thread::spawn(move || {
            if let Err(e) = hostapd_socket(tx_clone, map_clone) {
                eprintln!("Error en hostapd_socket: {}", e);
            }
        });
    }

    // Hilo principal: escucha eventos y actúa
    loop {
        if let Ok(event) = rx.recv_timeout(Duration::from_secs(1)) {
            match event {
                HostapdEvent::Connected(mac) => {
                    println!("[+] Conectado: {}", mac);

                    if let Ok(leases) = parse_leases() {
                        if let Some(ip) = leases.get(&mac) {
                            println!("→ IP asignada: {}", ip);
                            let ip_clone = ip.to_string();
                            let mac_clone = mac.clone();
                            // Ejecutar la función async en un runtime temporal
                            if let Ok(authorized) = tokio::runtime::Runtime::new()
                                .unwrap()
                                .block_on(backend_http(ip_clone, mac_clone))
                            {
                                if authorized {
                                    allow_ip(ip, &allowed_ips);
                                    println!("→ IP autorizada: {}", ip);
                                }
                            }
                            // Ejecutar lógica de autorización, iptables, etc.
                        }
                    }
                }
                HostapdEvent::Disconnected(mac) => {
                    println!("[-] Desconectado: {}", mac);
                    if let Some(ip) = mac_ip_map.get(&mac) {
                        block_ip(&ip, &allowed_ips);
                        println!("→ IP bloqueada: {}", ip.value());
                    }
                }
                HostapdEvent::Other(msg) => {
                    println!("[hostapd] {}", msg);
                }
            }
        }
    }
}
