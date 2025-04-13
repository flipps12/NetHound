use clap::{Parser, Subcommand};
use std::collections::HashMap;
use std::process::{Child, Command, Stdio};

#[derive(Parser)]
#[command(name = "NetHound")]
#[command(about = "Launcher de servicios NetHound", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Inicia todos los servicios
    Start,
    /// Detiene todos los servicios
    Stop,
    /// Muestra el estado de los procesos
    Status,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Start => start_services(),
        Commands::Stop => stop_services(),
        Commands::Status => status_services(),
    }
}

fn start_services() {
    println!("🚀 Iniciando servicios NetHound...");

    let mut handles: HashMap<&str, Child> = HashMap::new();
    // Desbloquea WiFi
    Command::new("rfkill")
        .arg("unblock")
        .arg("wifi")
        .status()
        .expect("⚠️ No se pudo desbloquear WiFi");

    // Configura dirección IP para wlan0
    Command::new("ip")
        .args(&["addr", "add", "192.168.1.1/24", "dev", "wlan0"])
        .status()
        .expect("⚠️ No se pudo configurar la dirección IP para wlan0");

    // Activa modo promiscuo en wlan0
    Command::new("ip")
        .args(&["link", "set", "wlan0", "promisc", "on"])
        .status()
        .expect("⚠️ No se pudo activar el modo promiscuo en wlan0");

    // Reinicia hostapd
    Command::new("systemctl")
        .args(&["restart", "hostapd"])
        .status()
        .expect("⚠️ No se pudo reiniciar hostapd");

    // Reinicia dnsmasq
    Command::new("systemctl")
        .args(&["restart", "dnsmasq"])
        .status()
        .expect("⚠️ No se pudo reiniciar dnsmasq");
    // Inicia Firewall
    let firewall = Command::new("/usr/local/bin/nethound/firewall")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("⚠️ No se pudo iniciar firewall");
    handles.insert("Firewall", firewall);

    // Inicia PacketAnalyzer
    let analyzer = Command::new("/usr/local/bin/nethound/PacketAnalyzer")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("⚠️ No se pudo iniciar PacketAnalyzer");
    handles.insert("PacketAnalyzer", analyzer);

    // Inicia backend (Node.js)
    let backend = Command::new("node")
        .arg("/usr/local/bin/nethound/backend/dist/index.js")
        .spawn()
        .expect("⚠️ No se pudo iniciar backend");
    handles.insert("Backend", backend);

    // Inicia frontend (serve -s build)
    let frontend = Command::new("serve")
        .arg("-s")
        .arg("/usr/local/bin/nethound/frontend/dist")
        .spawn()
        .expect("⚠️ No se pudo iniciar frontend");
    handles.insert("Frontend", frontend);

    println!("✅ Todos los servicios han sido lanzados.");
}

fn stop_services() {
    println!(
        "🛑 Detener servicios: esta función aún no implementa manejo de PIDs. Usa systemctl o pkill."
    );
}

fn status_services() {
    println!("🔍 Estado: esta versión aún no implementa monitoreo.");
}
