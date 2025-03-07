use crate::core::event_bus::{Event, EventBus};
use crate::core::packet_data::PacketData;
use crate::core::packet_processor::PacketProcessor;
use crate::core::utils::get_interfaces::get_interfaces;

use colored::Colorize;
use std::io::{self, Write};
use std::process::Command; // , ptr::null
use std::sync::Arc;
use tokio::task;

fn switch_mode(mode: i16, interface_w: String, interface_l: String) {
    match mode {
        1 => {
            let commands = [
                "sysctl -w net.ipv4.ip_forward=1",
                
                // Limpiar las reglas previas de iptables
                "iptables -t nat -F",
                "iptables -F",
                "iptables -X",
                
                // Crear el bridge si no existe
                "ip link add name br0 type bridge || true",
                
                // Detener servicios potencialmente conflictivos
                "systemctl stop dnsmasq",
    
                // Asignar interfaces al bridge
                &format!("ip link set {} master br0", interface_w),
                &format!("ip link set {} master br0", interface_l),
    
                // Asignar IP al bridge y habilitar la interfaz
                "ip addr flush dev br0",
                "ip addr add 192.168.0.14/24 dev br0",
                "ip link set br0 up",

                // Configuración de NAT y reenvío de paquetes
                &format!("iptables -t nat -A POSTROUTING -o {} -j MASQUERADE", interface_l),
                &format!("iptables -A FORWARD -i {} -o {} -j ACCEPT", interface_w, interface_l),
                &format!("iptables -A FORWARD -i {} -o {} -m state --state RELATED,ESTABLISHED -j ACCEPT", interface_l, interface_w),
            ];
    
            for cmd in commands.iter() {
                Command::new("sh")
                    .arg("-c")
                    .arg(cmd)
                    .spawn()
                    .expect("Error starting Bridge Mode");
            }
        }
        2 => {
            let commands = [
                "sysctl -w net.ipv4.ip_forward=1",
                "iptables -t nat -F",
                "iptables -F",
                "iptables -X",
                
                // Configuración para modo aislado
                &format!("ip addr flush dev {}", interface_w),
                
                // Asignar IP estática directamente a wlan0
                &format!("ip addr add 192.168.10.1/24 dev {}", interface_w),
                &format!("ip link set {} up", interface_w),
                
                // Configurar NAT y reglas de iptables
                &format!("iptables -t nat -A POSTROUTING -o {} -j MASQUERADE", interface_l),
                &format!("iptables -A FORWARD -i {} -o {} -j ACCEPT", interface_w, interface_l),
                &format!("iptables -A FORWARD -i {} -o {} -m state --state RELATED,ESTABLISHED -j ACCEPT", interface_l, interface_w),
                
                // Aislamiento total de la red Wi-Fi
                &format!("iptables -A FORWARD -i {} -o {} -j DROP", interface_w, interface_w),
            ];
    
            for cmd in commands.iter() {
                Command::new("sh")
                    .arg("-c")
                    .arg(cmd)
                    .spawn()
                    .expect("Error starting Isolated Mode");
            }
        }
        _ => {
            println!("Invalid mode");
        }
    }
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

    // Define NetLab mode
    let mut buffer2 = String::new();
    print!("    {} - ", "1".cyan());
    print!("Bridge Mode\n");
    print!("    {} - ", "2".cyan());
    print!("Isolated Mode\n");
    print!("Select NetLab mode: ");
    io::stdout().flush().expect("Error at flushing stdout");
    io::stdin()
        .read_line(&mut buffer2)
        .expect("Error at reading input");
    let mode: i16 = buffer2.trim().parse::<i16>().unwrap();
    switch_mode(mode, "wlan0".to_string(), "eth0".to_string());

    // Create shared EventBus
    let event_bus = Arc::new(EventBus::new());
    let mut subscriber = event_bus.subscribe();

    // Task to process and display received events
    let interface_name_clone = interface_name.clone();
    tokio::spawn(async move {
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
                    }
                    if let Some(udp_src) = data.udp_src_port {
                        println!(
                            "  UDP: {} -> {}",
                            udp_src.to_string().green(),
                            data.udp_dst_port.unwrap_or(0).to_string().green()
                        );
                        println!("  Length: {:?}", data.udp_length);
                    }
                }
                Event::DropPacket(data) => {
                    println!("Event: Drop packet request:");
                    update_firewall(&data);
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
