use std::path::PathBuf;

use crate::solver::CliSolver;

#[derive(Debug, clap::Subcommand)]
pub enum CliCommand {
    /// Solve challenges using provided files as inputs
    Solve {
        /// Variant of solver to use
        #[arg(short, long, required = true)]
        solver: CliSolver,
        /// Inputs of the challenges to solve
        #[arg(required = true)]
        files: Vec<PathBuf>,
    },
}
