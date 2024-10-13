use std::{fs, io::BufReader, path::PathBuf, str::FromStr};

use aoc_traits::error::{AocError, AocResult};

use super::ChallengeFile;

#[derive(clap::Parser)]
#[command(version, author, about, arg_required_else_help(true))]
pub struct Cli {
    /// A challenge to solve. Format: <year>-<day>-<part>:<input-path>
    #[arg(short, long = "challenge", required_unless_present = "args_file")]
    challenges: Vec<ChallengeFile>,
    /// A json file with a list of challenges to solve.
    /// Content is as follow: [ "<year>-<day>-<part>:<input-path>" ]
    #[arg(short, long)]
    args_file: Option<PathBuf>,
}

impl Cli {
    pub fn challenges(&self) -> &[ChallengeFile] {
        &self.challenges
    }

    pub fn into_challenges(self) -> impl Iterator<Item = ChallengeFile> {
        self.challenges.into_iter()
    }

    pub fn file_challenges(&self) -> AocResult<Vec<ChallengeFile>> {
        self.args_file
            .as_ref()
            .map(|path| -> AocResult<Vec<ChallengeFile>> {
                let reader = fs::File::open(path)?;
                let lines = serde_json::from_reader::<_, Vec<String>>(BufReader::new(reader))
                    .map_err(AocError::parsing)?;

                lines
                    .into_iter()
                    .map(|line| ChallengeFile::from_str(&line))
                    .collect::<AocResult<Vec<ChallengeFile>>>()
            })
            .unwrap_or_else(|| Ok(Vec::default()))
    }
}
