mod core {
    pub mod event_bus;
    pub mod packet_data;
    pub mod packet_processor;
    pub mod utils {
        pub mod get_interfaces;
    }
}
mod modes {
    pub mod netlab;
}
mod launcher;

use core::*;

fn main() {
    println!("Hello, world!");
    launcher::launch();
}