use tokio::sync::broadcast;

use crate::packet_data::PacketData;

#[derive(Debug, Clone)]
pub enum Event {
    PacketReceived(PacketData),
    DropPacket(PacketData),
}

pub struct EventBus {
    sender: broadcast::Sender<Event>,
}

impl EventBus {
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(100);
        Self { sender }
    }

    pub async fn emit(&self, event: Event) {
        let _ = self.sender.send(event);
    }

    pub fn subscribe(&self) -> broadcast::Receiver<Event> {
        self.sender.subscribe()
    }
}