use std::{thread, time::Duration};
use clap::{Parser, Subcommand};
use rppal::gpio::Gpio;
use std::collections::HashMap;
use std::process::{Child, Command, Stdio};
use std::io::{BufRead, BufReader};
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
    println!("Iniciando servicios internos de NetHound...");

    let mut handles: HashMap<&str, Child> = HashMap::new();
    let pin1 = 17;
    let pin2 = 22;
    let pin3 = 23;
    let pin4 = 27;
    let mut led1 = Gpio::new()
        .expect("Error al inicializar GPIO")
        .get(pin1)
        .expect("Error al obtener el pin")
        .into_output();
    let mut led2 = Gpio::new()
        .expect("Error al inicializar GPIO")
        .get(pin2)
        .expect("Error al obtener el pin")
        .into_output();
    let mut led3 = Gpio::new()
        .expect("Error al inicializar GPIO")
        .get(pin3)
        .expect("Error al obtener el pin")
        .into_output();
    let mut led4 = Gpio::new()
        .expect("Error al inicializar GPIO")
        .get(pin4)
        .expect("Error al obtener el pin")
        .into_output();

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
    let mut firewall = Command::new("/usr/local/bin/nethound/firewall")
        //.stdout(Stdio::null())
        //.stderr(Stdio::null())
        .spawn()
        .expect("⚠️ No se pudo iniciar firewall");
    //handles.insert("Firewall", firewall);
    led1.set_high();

    thread::sleep(Duration::from_secs(3));
    // Inicia PacketAnalyzer
    let mut analyzer = Command::new("/usr/local/bin/nethound/PacketAnalyzer")
        //.stdout(Stdio::null())
        //.stderr(Stdio::null())
        .spawn()
        .expect("⚠️ No se pudo iniciar PacketAnalyzer");
    //handles.insert("PacketAnalyzer", analyzer);
    led2.set_high();

    // Inicia backend (Node.js)
    let mut backend = Command::new("node")
        .arg("/usr/local/bin/nethound/backend/dist/index.js")
        .spawn()
        .expect("⚠️ No se pudo iniciar backend");
    //handles.insert("Backend", backend);
    led3.set_high();

    // Inicia frontend (serve -s build)
    let mut frontend = Command::new("serve")
        .args(["-s", "/usr/local/bin/nethound/frontend/dist", "-l", "8080"])
        .spawn()
        .expect("⚠️ No se pudo iniciar frontend");
    //handles.insert("Frontend", frontend);
    led4.set_high();

    if let Some(stdout) = analyzer.stderr.take() {
        let reader = BufReader::new(stdout);
        std::thread::spawn(move ||
            for line in reader.lines() {
                if let Ok(I) = line {
                    eprintln!("[stdout] {}", I);
                }
            });
    }

    if let Some(stdout) = firewall.stdout.take() {
        let reader = BufReader::new(stdout);
        std::thread::spawn(move ||
            for line in reader.lines() {
                if let Ok(I) = line {
                    eprintln!("[stdout] {}", I);
                }
            });
    }

    println!("✅ Todos los servicios han sido lanzados.");
    std::thread::park();
}

fn stop_services() {
    println!(
        "🛑 Detener servicios: esta función aún no implementa manejo de PIDs. Usa systemctl o pkill."
    );
}

fn status_services() {
    println!("🔍 Estado: esta versión aún no implementa monitoreo.");
}
