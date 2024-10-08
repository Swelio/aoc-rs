use std::fmt::{self, Display};

use crate::{
    challenge::{identity::Identity, input::InputName},
    error::{AocError, AocResult},
};

use super::Flag;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(remote = "AocResult<Flag>")]
#[serde(tag = "kind")]
enum SerdeResult {
    Ok(Flag),
    Err(AocError),
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Solution {
    identity: Identity,
    input_name: InputName,
    #[serde(with = "SerdeResult")]
    resolution: AocResult<Flag>,
}

impl Solution {
    pub fn new(identity: Identity, input_name: InputName, resolution: AocResult<Flag>) -> Self {
        Self {
            identity,
            input_name,
            resolution,
        }
    }

    pub fn identity(&self) -> Identity {
        self.identity
    }

    pub fn input_name(&self) -> &InputName {
        &self.input_name
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
