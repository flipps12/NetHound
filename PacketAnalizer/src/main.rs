mod core {
    pub mod event_bus;
    pub mod packet_data;
    pub mod packet_processor;
    pub mod utils {
        pub mod get_interfaces;
    }
}

use crate::core::event_bus::{Event, EventBus};
use crate::core::packet_data::PacketData;
use crate::core::packet_processor::PacketProcessor;
use crate::core::utils::get_interfaces::get_interfaces;

use colored::Colorize;
use rppal::gpio::Gpio;
use std::io::{self, Write};
use std::os::unix::net::UnixStream;
use std::process::Command; // , ptr::null
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::task;

fn main() {
    print!("\x1B[2J\x1B[H");
    println!(
        "{}",
        format!("NetHound -  {}", env!("CARGO_PKG_VERSION"))
            .bold()
            .cyan()
    );
    run();
}

fn update_firewall(packet: &PacketData) {
    if let Some(ref src_ip) = packet.src_ip {
        println!("Blocking traffic from IP: {}", src_ip);
        let output = Command::new("sudo")
            .args(&["iptables", "-I", "INPUT", "-s", src_ip, "-j", "DROP"])
            .output();
        match output {
            Ok(o) => println!("Firewall updated: {:?}", String::from_utf8_lossy(&o.stdout)),
            Err(e) => eprintln!("Error updating firewall: {:?}", e),
        }
    } else {
        println!("Could not determine source IP; firewall not updated.");
    }
}

fn get_temp() -> String {
    let output = Command::new("sensors")
        .output()
        .expect("Error executing command");

    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        let temp = output_str
            .lines()
            .find(|line| line.contains("temp1:"))
            .map(|line| line.split_whitespace().last().unwrap())
            .unwrap_or("N/A");

        temp.to_string()
    } else {
        eprintln!("Error executing 'sensors': {:?}", output);
        "N/A".to_string()
    }
}

pub fn restore_firewall() {
    let restore_cmds = [
        "iptables-restore < /etc/iptables/rules.v4",
        "ip6tables-restore < /etc/iptables/rules.v6",
        // "ip link set br0 down 2>/dev/null",
        // "ip link delete br0 type bridge 2>/dev/null",
        // "ip addr flush dev wlan0",
        // "ip addr flush dev eth0",
    ];

    for cmd in restore_cmds {
        let output = Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .output()
            .expect("Error executing command");

        if !output.status.success() {
            eprintln!("Error executing '{}': {:?}", cmd, output);
        }
    }

    println!("🔄 Firewall rules restored successfully");
}

#[tokio::main]
pub async fn run() {
    // Initialize GPIO

    let pin1 = 26;
    let pin2 = 20;
    let pin3 = 21;
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

    // sockets
    let stream = Arc::new(Mutex::new(
        UnixStream::connect("/tmp/net_hound.sock").expect("No se pudo conectar al socket UNIX"),
    ));
    let (tx, mut rx) = mpsc::channel::<String>(100);

    // Define network interface to capture packets
    let interfaces = get_interfaces();
    let mut i = 1;
    for interface in &interfaces {
        print!("    {} - ", i.to_string().green());
        print!("{} \n", interface.cyan());
        i += 1;
    }
    print!("Select interface: ");
    io::stdout().flush().expect("Error at flushing stdout");
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Error at reading input");
    let interface_index: usize = buffer.trim().parse::<usize>().unwrap();
    let interface_name = interfaces[interface_index - 1].clone();

    // restore_firewall();

    // Create shared EventBus
    let event_bus = Arc::new(EventBus::new());
    let mut subscriber = event_bus.subscribe();

    // Task to process and display received events
    let interface_name_clone = interface_name.clone();
    let stream_clone = Arc::clone(&stream);
    task::spawn(async move {
        while let Some(message) = rx.recv().await {
            let mut stream = stream_clone.lock().unwrap();
            if let Err(e) = stream.write_all(message.as_bytes()) {
                eprintln!("⚠ Error enviando datos: {:?}", e);
                break; // Detenemos si el socket se rompe
            }
        }
    });

    tokio::spawn({
        async move {
            while let Ok(event) = subscriber.recv().await {
                print!("\x1B[2J\x1B[H");
                println!(
                    "Intercepting packet in {}",
                    format!("{}", interface_name_clone).green()
                );
                println!("CPU Temperature: {}", get_temp().red());
                match event {
                    Event::PacketReceived(data) => {
                        println!("Event: Packet received:");
                        println!("  Source MAC: {:?}", data.src_mac);
                        println!("  Destination MAC: {:?}", data.dst_mac);
                        println!("  EtherType: {:?}", data.ethertype);
                        println!(
                            "  Source IP: {}",
                            data.src_ip
                                .as_ref()
                                .map_or("None", |ip| ip)
                                .to_string()
                                .purple()
                        );
                        println!(
                            "  Destination IP: {}",
                            data.dst_ip
                                .as_ref()
                                .map_or("None", |ip| ip)
                                .to_string()
                                .purple()
                        );
                        println!("  IP Protocol: {:?}", data.ip_protocol);
                        if let Some(tcp_src) = data.tcp_src_port {
                            println!(
                                "  TCP: {} -> {}",
                                tcp_src.to_string().green(),
                                data.tcp_dst_port.unwrap_or(0).to_string().green()
                            );
                            println!("  Sequence: {:?}", data.tcp_sequence);
                            println!("  Acknowledgment: {:?}", data.tcp_ack);
                            println!("  Flags: {:?}", data.tcp_flags);
                            led2.set_high();
                        }

                        // ✅ Formateamos el mensaje y lo enviamos al canal
                        if let (Some(src_ip), Some(src_mac)) =
                            (data.src_ip.as_ref(), data.src_mac.as_deref())
                        {
                            let message = format!("{}\n{}\n", src_ip, src_mac);
                            if let Err(e) = tx.send(message).await {
                                eprintln!("⚠ Error enviando al canal: {:?}", e);
                            }
                            led1.set_high();
                        }

                        // Toggle LED on packet received
                        led3.set_high();
                        sleep(Duration::from_millis(100));
                        led1.set_low();
                        led2.set_low();
                        led3.set_low();
                    }
                    Event::DropPacket(data) => {
                        println!("Event: Drop packet request:");
                        update_firewall(&data);
                    }
                }
            }
        }
    });

    // Create processor for the WLAN interface (e.g., "wlan0")
    let processor = PacketProcessor::new(Arc::clone(&event_bus), &interface_name);
    // Run packet capture in a blocking context to avoid blocking the Tokio runtime
    task::spawn_blocking(move || {
        processor.run();
    })
    .await
    .unwrap();
}
