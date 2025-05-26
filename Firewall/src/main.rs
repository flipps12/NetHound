use dashmap::DashMap;
use std::{fs, thread};
use std::{net::UdpSocket, path::Path};
use std::process::Command;
use std::sync::Arc;
use tokio::{io::{AsyncBufReadExt, AsyncWriteExt}, net::{UnixListener, UnixStream}};
use std::os::unix::net::UnixDatagram;
use tokio::{
    io::BufReader,
    time::{self, Duration},
};
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::sync::mpsc;

const SERVER_WEB: &str = "http://localhost/api";
const CHECK_INTERVAL: Duration = Duration::from_secs(5);

#[derive(Debug)]
enum HostapdEvent {
    Connected(String),    // MAC
    Disconnected(String), // MAC
    Other(String),        // Texto completo
}

#[derive(Debug)]
struct Firewall {
    allowed_ips: Arc<DashMap<String, bool>>, // IP -> Autorizada o no
}

fn run_command(command: &str, args: &[&str]) {
    let status = Command::new("sudo")
        .arg(command)
        .args(args)
        .status()
        .expect("Error ejecutando el comando");
    if status.success() {
        println!("✅ Comando ejecutado: {} {:?}", command, args);
    } else {
        eprintln!("❌ Error ejecutando: {} {:?}", command, args);
    }
}

fn get_private_ip() -> String {
    let socket = UdpSocket::bind("0.0.0.0:0").expect("No se pudo crear el socket");
    socket
        .connect("8.8.8.8:80")
        .expect("No se pudo conectar al socket");
    if let Ok(local_addr) = socket.local_addr() {
        return local_addr.ip().to_string();
    }
    "0.0.0.0".to_string()
}


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


fn hostapd_socket(tx: Sender<HostapdEvent>) -> std::io::Result<()> {
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
                tx.send(HostapdEvent::Connected(mac.to_string())).ok();
            }
        } else if msg.contains("AP-STA-DISCONNECTED") {
            if let Some(mac) = msg.split_whitespace().last() {
                tx.send(HostapdEvent::Disconnected(mac.to_string())).ok();
            }
        } else {
            tx.send(HostapdEvent::Other(msg.to_string())).ok();
        }
    }
}


fn main() -> std::io::Result<()> {
    let (tx, rx) = mpsc::channel();

    // 🧵 Hilo para hostapd
    thread::spawn(move || {
        if let Err(e) = hostapd_socket(tx) {
            eprintln!("[!] Error en hostapd_socket: {}", e);
        }
    });

    // Hilo principal: escucha eventos y actúa
    loop {
        if let Ok(event) = rx.recv_timeout(Duration::from_secs(1)) {
            match event {
                HostapdEvent::Connected(mac) => {
                    println!("[+] Conectado: {}", mac);

                    if let Ok(leases) = parse_leases() {
                        if let Some(ip) = leases.get(&mac) {
                            println!("→ IP asignada: {}", ip);
                            // Ejecutar lógica de autorización, iptables, etc.
                        }
                    }
                }
                HostapdEvent::Disconnected(mac) => {
                    println!("[-] Desconectado: {}", mac);
                }
                HostapdEvent::Other(msg) => {
                    println!("[hostapd] {}", msg);
                }
            }
        }
    }
}