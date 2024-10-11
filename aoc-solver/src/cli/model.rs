use super::ChallengeFile;

#[derive(clap::Parser)]
#[command(version, author, about, arg_required_else_help(true))]
pub struct Cli {
    #[arg(short, long = "challenge", num_args = 1.., help = "<year>-<day>-<part>:<input-path>")]
    challenges: Vec<ChallengeFile>,
}

impl Cli {
    pub fn challenges(&self) -> &[ChallengeFile] {
        &self.challenges
    }

    pub fn into_challenges(self) -> impl Iterator<Item = ChallengeFile> {
        self.challenges.into_iter()
    }
}
