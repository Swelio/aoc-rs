#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum CliSolver {
    All,
    #[value(name = "2015")]
    Year2015,
}
