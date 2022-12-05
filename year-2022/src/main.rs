#![deny(clippy::all)]

use std::process::exit;

use clap::Parser;

use year_2022::Command;

fn main() {
    let command: Command = Command::parse();

    if let Err(err) = command.run() {
        eprintln!("An error occurred: {}", err);
        exit(1);
    }
}
