use dashmap::DashMap;
use std::io::Read;
use std::net::IpAddr;
use std::os::unix::net::UnixListener;
use std::process::Command;
use std::sync::Arc;
use tokio::time::{self, Duration};
use std::net::UdpSocket;

const SERVER_WEB: &str = "http://localhost/api";
const CHECK_INTERVAL: Duration = Duration::from_secs(5);

#[derive(Debug)]
struct Firewall {
    allowed_ips: Arc<DashMap<String, bool>>, // IP -> Autorizada o no
}

impl Firewall {
    fn new() -> Self {
        let firewall = Firewall {
            allowed_ips: Arc::new(DashMap::new()),
        };
        firewall.block_all();
        firewall
    }

    fn block_all(&self) {
        // Política por defecto: bloquear todo el tráfico reenviado
        run_command("iptables", &["-P", "FORWARD", "DROP"]);

        // Eliminar reglas existentes en FORWARD
        run_command("iptables", &["-F", "FORWARD"]);

        // Permitir tráfico de retorno de conexiones válidas
        run_command(
            "iptables",
            &[
                "-A",
                "FORWARD",
                "-m",
                "conntrack",
                "--ctstate",
                "RELATED,ESTABLISHED",
                "-j",
                "ACCEPT",
            ],
        );

        // Habilitar NAT para compartir internet
        run_command(
            "iptables",
            &[
                "-t",
                "nat",
                "-A",
                "POSTROUTING",
                "-o",
                "eth0",
                "-j",
                "MASQUERADE",
            ],
        );

        println!(
            "🚫 Todo el tráfico está bloqueado salvo IPs autorizadas y conexiones existentes."
        );
    }

    fn is_private_ip(ip: &str) -> bool {
        matches!(ip.parse::<IpAddr>(), Ok(IpAddr::V4(v4)) if v4.is_private())
    }

    async fn check_authorization(&self, mac: &str, ip: &str) -> bool {
        let url = format!("{}/verify?mac={}&ip={}", SERVER_WEB, mac, ip);
        println!("🔗 Consultando: {}", url);

        if let Ok(response) = reqwest::get(&url).await {
            if let Ok(body) = response.text().await {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
                    if let Some(authorized) = json.get("authorized").and_then(|v| v.as_bool()) {
                        println!("🔑 Autorizado: {}", authorized);
                        self.allowed_ips.insert(ip.to_string(), authorized);
                        if authorized {
                            self.unblock_ip(ip);
                        }
                        return authorized;
                    }
                }
            }
        }
        false
    }

    async fn check_authorization_ip(&self, ip: &str) -> bool {
        let url = format!("{}/onlyip?ip={}", SERVER_WEB, ip);

        if let Ok(response) = reqwest::get(&url).await {
            if let Ok(body) = response.text().await {
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&body) {
                    if let Some(authorized) = json.get("authorized").and_then(|v| v.as_bool()) {
                        println!("🔑 Autorizado: {}", authorized);
                        self.allowed_ips.insert(ip.to_string(), authorized);
                        if authorized {
                            self.unblock_ip(ip);
                        } else {
                            self.block_ip(ip);
                        }
                        return authorized;
                    }
                }
            }
        }
        false
    }

    fn unblock_ip(&self, ip: &str) {
        // Eliminamos posibles reglas repetidas previas
        let _ = run_command(
            "iptables",
            &[
                "-D", "FORWARD", "-s", ip, "-i", "wlan0", "-o", "eth0", "-j", "ACCEPT",
            ],
        );

        // Insertamos la nueva regla
        run_command(
            "iptables",
            &[
                "-I", "FORWARD", "-s", ip, "-i", "wlan0", "-o", "eth0", "-j", "ACCEPT",
            ],
        );

        println!("🚀 IP autorizada para pasar de wlan0 a eth0: {}", ip);
    }

    fn block_ip(&self, ip: &str) {
        run_command(
            "iptables",
            &[
                "-D", "FORWARD", "-s", ip, "-i", "wlan0", "-o", "eth0", "-j", "ACCEPT",
            ],
        );
        println!(
            "⛔️ IP bloqueada (acceso desde wlan0 a eth0 denegado): {}",
            ip
        );
    }
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

#[tokio::main]
async fn main() {
    let socket_path = "/tmp/net_hound.sock";
    let firewall = Arc::new(Firewall::new());

    println!("IP privada: {}", get_private_ip());

    if std::path::Path::new(socket_path).exists() {
        std::fs::remove_file(socket_path).unwrap();
    }

    let listener = UnixListener::bind(socket_path).unwrap();
    println!("Servidor escuchando en: {}", socket_path);

    let firewall_clone = Arc::clone(&firewall);
    tokio::spawn(async move {
        println!("🔄 Iniciando verificación de IPs...");
        let mut interval = time::interval(CHECK_INTERVAL);
        loop {
            interval.tick().await;
            println!("🔄 {:?}", firewall_clone.allowed_ips);
            let ips: Vec<String> = firewall_clone
                .allowed_ips
                .iter()
                .map(|entry| entry.key().clone())
                .collect();
            for ip in ips {
                println!(
                    "🔄 Verificando IP: {:?}",
                    firewall_clone.check_authorization_ip(&ip).await
                );
            }
        }
    });

    loop {
        match listener.accept() {
            Ok((mut stream, _)) => {
                let firewall_clone = Arc::clone(&firewall);
                tokio::spawn(async move {
                    let mut buffer = [0; 1024];
                    if let Ok(size) = stream.read(&mut buffer) {
                        let received = String::from_utf8_lossy(&buffer[..size]);
                        let lines: Vec<&str> = received.lines().collect();
                        println!("🔗 Recibido: {}\n\n\n", received);
                        if lines.len() >= 2 {
                            let ip = lines[0];
                            let mac = lines[1];

                            println!("🔗 IP: {} MAC: {}", ip, mac);

                            if Firewall::is_private_ip(ip) && ip != "192.168.1.1" && ip != "0.0.0.0" && ip != get_private_ip().as_str()
                            {
                                println!("🔗 IP: {} MAC: {}", ip, mac);
                                if firewall_clone.allowed_ips.contains_key(ip) {
                                    if *firewall_clone.allowed_ips.get(ip).unwrap() {
                                        firewall_clone.unblock_ip(ip);
                                    }
                                } else {
                                    firewall_clone.check_authorization(mac, ip).await;
                                    println!("New ip: {}", ip);
                                }
                            }
                        }
                    }
                });
            }
            Err(e) => {
                eprintln!("❌ Error al aceptar conexión: {}", e);
            }
        }
    }
}
