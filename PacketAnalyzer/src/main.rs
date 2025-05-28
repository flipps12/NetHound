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

use chrono::prelude::*;
use colored::Colorize;
use r2d2_sqlite::rusqlite::{self, Connection, params};
use std::fs;
// use std::path::Path;
use std::sync::{Arc, mpsc};
use tokio::task;

#[derive(Debug, Clone)]
pub struct PacketLog {
    pub timestamp: String,
    pub src_ip: String,
    pub dst_ip: String,
    pub src_mac: String,
    pub dst_mac: String,
    pub protocol: String,
    pub bytes: usize,
    pub count: usize,
}

fn get_db_path() -> String {
    let now = Utc::now().format("%Y-%m-%d").to_string();
    let dir = "/var/lib/nethound";
    fs::create_dir_all(dir).unwrap();
    format!("{}/traffic_{}.db", dir, now)
}

pub fn create_table(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.pragma_update(None, "journal_mode", &"WAL")?;
    conn.pragma_update(None, "synchronous", &"NORMAL")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS packet_summary (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            src_ip TEXT NOT NULL,
            dst_ip TEXT NOT NULL,
            src_mac TEXT NOT NULL,
            dst_mac TEXT NOT NULL,
            protocol TEXT NOT NULL,
            bytes INTEGER NOT NULL,
            count INTEGER NOT NULL,
            first_seen TEXT NOT NULL,
            last_seen TEXT NOT NULL,
            UNIQUE(src_ip, dst_ip, src_mac, dst_mac, protocol)
        );",
        [],
    )?;
    Ok(())
}

pub fn upsert_packet_summary(conn: &Connection, log: &PacketLog) {
    conn.execute(
        "INSERT INTO packet_summary (src_ip, dst_ip, src_mac, dst_mac, protocol, bytes, count, first_seen, last_seen)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?8)
         ON CONFLICT(src_ip, dst_ip, src_mac, dst_mac, protocol)
         DO UPDATE SET
            bytes = bytes + excluded.bytes,
            count = count + excluded.count,
            last_seen = excluded.last_seen;",
        params![
            log.src_ip,
            log.dst_ip,
            log.src_mac,
            log.dst_mac,
            log.protocol,
            log.bytes as i64,
            log.count as i64,
            log.timestamp,
        ],
    )
    .unwrap();
}

fn save_packet_summary(packet: &PacketData) -> PacketLog {
    let timestamp = chrono::Utc::now().to_rfc3339();
    let binding = "None".to_string();
    let src_mac = packet.src_mac.as_ref().unwrap_or(&binding).clone();
    let dst_mac = packet.dst_mac.as_ref().unwrap_or(&binding).clone();
    let src_ip = packet.src_ip.as_ref().unwrap_or(&binding).clone();
    let dst_ip = packet.dst_ip.as_ref().unwrap_or(&binding).clone();

    let protocol = if packet.tcp_src_port.is_some() {
        "TCP"
    } else if packet.udp_src_port.is_some() {
        "UDP"
    } else {
        "Other"
    };

    PacketLog {
        timestamp,
        src_ip,
        dst_ip,
        src_mac,
        dst_mac,
        protocol: protocol.to_string(),
        bytes: packet._raw_data.len(),
        count: 1,
    }
}

fn main() {
    print!("\x1B[2J\x1B[H");
    println!(
        "{}",
        format!("NetHound -  {}", env!("CARGO_PKG_VERSION"))
            .bold()
            .cyan()
    );
    let _ = run();
}

#[tokio::main]
pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let (tx_db, rx_db) = mpsc::channel::<PacketLog>();

    std::thread::spawn(move || {
        let mut current_db_path = get_db_path();
        let mut conn = Connection::open(&current_db_path).expect("Failed to open DB");
        create_table(&conn).expect("Failed to create table");

        while let Ok(log) = rx_db.recv() {
            let db_path = get_db_path();
            if db_path != current_db_path {
                current_db_path = db_path;
                conn = Connection::open(&current_db_path).expect("Failed to open new DB");
                create_table(&conn).expect("Failed to create table");
            }

            upsert_packet_summary(&conn, &log);
        }
    });

    let log = false;
    let interface_name = "wlan0";
    let event_bus = Arc::new(EventBus::new());
    let mut subscriber = event_bus.subscribe();

    tokio::spawn({
        let tx_db = tx_db.clone();

        async move {
            while let Ok(event) = subscriber.recv().await {
                // print!("\x1B[2J\x1B[H");
                if log {
                    println!(
                        "Intercepting packet in {}",
                        format!("{}", interface_name).green()
                    );
                }

                match event {
                    Event::PacketReceived(data) => {
                        if log {
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
                        }

                        let summary = save_packet_summary(&data);

                        if log {
                            println!(
                                "Packet received: {} -> {} ({} bytes)",
                                summary.src_ip.cyan(),
                                summary.dst_ip.cyan(),
                                summary.bytes.to_string().green()
                            );
                        }

                        if let Err(e) = tx_db.send(summary) {
                            eprintln!("⚠ Error enviando paquete al hilo DB: {:?}", e);
                        }
                    }
                    Event::DropPacket(_data) => {
                        println!("Event: Drop packet request:");
                    }
                }
            }
        }
    });

    let processor = PacketProcessor::new(Arc::clone(&event_bus), &interface_name);
    task::spawn_blocking(move || {
        processor.run();
    })
    .await
    .unwrap();

    Ok(())
}
