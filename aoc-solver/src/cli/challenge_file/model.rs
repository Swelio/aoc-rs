use std::path::{Path, PathBuf};

use aoc_traits::challenge::Identity;

#[derive(Debug, Clone)]
pub struct ChallengeFile {
    identity: Identity,
    file: PathBuf,
}

impl ChallengeFile {
    pub fn new(identity: Identity, file: PathBuf) -> Self {
        Self { identity, file }
    }

    pub fn identity(&self) -> Identity {
        self.identity
    }

    pub fn file(&self) -> &Path {
        &self.file
    }
}
