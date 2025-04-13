use std::process;
use std::sync::Arc;
use pnet::datalink::{self, Channel};
// use tokio::task;
use crate::core::event_bus::{Event, EventBus};
use crate::core::packet_data::parse_packet;

pub struct PacketProcessor {
    pub event_bus: Arc<EventBus>,
    pub interface_name: String,
}

impl PacketProcessor {
    pub fn new(event_bus: Arc<EventBus>, interface_name: &str) -> Self {
        Self {
            event_bus,
            interface_name: interface_name.to_string(),
        }
    }

    // Función bloqueante para capturar paquetes y emitir eventos
    pub fn run(&self) {
        let interfaces = datalink::interfaces();
        let interface = interfaces
            .into_iter()
            .find(|i| i.name == self.interface_name)
            .expect("Interfaz no encontrada");
        println!("Iniciando captura en la interfaz: {}...", self.interface_name);

        let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
            Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => {
                eprintln!("Canal no soportado");
                process::exit(1);
            }
            Err(e) => {
                eprintln!("Error al abrir canal: {}", e);
                process::exit(1);
            }
        };

        loop {
            match rx.next() {
                Ok(packet) => {
                    let packet_data = parse_packet(packet);
                    let event_bus = Arc::clone(&self.event_bus);

                    // test
                    if let Some(ip) = packet_data.src_ip.clone() {
                        if ip == "192.168.1.10300" {
                            let packet_data_clone = packet_data.clone();
                            // Emitir evento para descartar el paquete
                            tokio::spawn(async move {
                                event_bus.emit(Event::DropPacket(packet_data_clone)).await;
                            });
                            continue; // Saltar el procesamiento adicional del paquete
                        }
                    }
                    // Fin test

                    tokio::spawn(async move {
                        event_bus.emit(Event::PacketReceived(packet_data)).await;
                    }
                );
                }
                Err(e) => eprintln!("Error capturando paquete: {}", e),
            }
        }
    }
}
