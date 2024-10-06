use std::{
    cmp::Ordering,
    fmt::{self, Display},
};

use crate::{dynamic_solver::Identity, AocError, Solution};

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct ChallengeResult {
    id: Identity,
    issue: ChallengeIssue,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
enum ChallengeIssue {
    Success(Solution),
    Failure(String),
}

impl ChallengeResult {
    pub fn success(id: Identity, solution: Solution) -> Self {
        Self {
            id,
            issue: ChallengeIssue::Success(solution),
        }
    }

    pub fn failure(id: Identity, error: AocError) -> Self {
        Self {
            id,
            issue: ChallengeIssue::Failure(error.to_string()),
        }
    }
}

impl Display for ChallengeResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.issue {
            ChallengeIssue::Success(solution) => write!(f, "{0}: {solution}", self.id),
            ChallengeIssue::Failure(aoc_error) => write!(f, "{0} - error: {aoc_error}", self.id),
        }
    }
}

impl PartialOrd for ChallengeResult {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ChallengeResult {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id.cmp(&other.id)
    }
}
