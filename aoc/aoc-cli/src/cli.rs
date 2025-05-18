use crate::command::CliCommand;

#[derive(Debug, clap::Parser)]
#[command(version, author, about, long_about = None)]
#[command(arg_required_else_help = true, propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    command: CliCommand,
}

impl Cli {
    pub fn run(&self) {
        if let Err(err) = self.command.run() {
            eprintln!("Command error:\n{err}");
        }
    }
}
