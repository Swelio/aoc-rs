use std::fmt::{self, Display};

use crate::{challenge::identity::Identity, error::AocResult};

use super::Flag;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Solution {
    identity: Identity,
    resolution: AocResult<Flag>,
}

impl Solution {
    pub fn new(identity: Identity, resolution: AocResult<Flag>) -> Self {
        Self {
            identity,
            resolution,
        }
    }

    pub fn identity(&self) -> Identity {
        self.identity
    }

    pub fn resolution(&self) -> &AocResult<Flag> {
        &self.resolution
    }

    pub fn is_sucess(&self) -> bool {
        self.resolution.is_ok()
    }

    pub fn is_failed(&self) -> bool {
        self.resolution.is_err()
    }
}

impl From<Solution> for AocResult<Flag> {
    fn from(solution: Solution) -> Self {
        solution.resolution
    }
}

impl Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let identity = self.identity;

        match &self.resolution {
            Ok(flag) => write!(f, "{identity}: {flag}"),
            Err(err) => write!(f, "{identity} - error: {err}"),
        }
    }
}
