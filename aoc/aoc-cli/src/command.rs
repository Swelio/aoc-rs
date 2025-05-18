mod solve;

use std::path::PathBuf;

#[derive(Debug, clap::Subcommand)]
pub enum CliCommand {
    /// Solve challenges using provided files as inputs
    Solve {
        /// Inputs of the challenges to solve
        #[arg(required = true)]
        files: Vec<PathBuf>,
    },
}

impl CliCommand {
    pub fn run(&self) -> anyhow::Result<()> {
        match self {
            CliCommand::Solve { files } => solve::run(files),
        }
    }
}
