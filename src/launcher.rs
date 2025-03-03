use crate::modes;

use colored::Colorize;
use std::io::{self, Write};

pub fn launch() {
    let modes = vec!["NetLab".green(), "NetSecure".red(), "NetCrack".red()];
    let mut i = 0;
    for mode in &modes {
        println!("  {}", format!("{} - {}", i.to_string().cyan(), mode).bold());
        i += 1;
    }
    print!("Select a mode: ");

    io::stdout().flush().expect("Error at flushing stdout");
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Error at reading input");
    let mode_index: usize = buffer.trim().parse::<usize>().unwrap();
    
    match mode_index {
        0 => modes::netlab::run(),
        _ => println!("Not implemented yet"),
    }

    print!("\n");
}
