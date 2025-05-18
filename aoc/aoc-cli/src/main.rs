use clap::Parser;
use cli::Cli;

mod cli;
mod command;

fn main() {
    let cli = Cli::parse();
    cli.run()
}
