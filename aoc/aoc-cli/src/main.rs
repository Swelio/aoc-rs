use clap::Parser;
use cli::Cli;

mod cli;
mod command;
mod solver;

fn main() {
    let _cli = Cli::parse();
}
