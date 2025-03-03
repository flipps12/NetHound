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

use colored::Colorize;

fn main() {
    print!("\x1B[2J\x1B[H");
    println!("{}", format!("NetHound -  {}", env!("CARGO_PKG_VERSION")).bold().cyan());
    launcher::launch();
}