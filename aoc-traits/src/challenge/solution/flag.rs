use core::fmt;

use nutype::nutype;
use serde::{Deserialize, Serialize};

#[nutype(derive(Debug, Display, Clone, PartialEq, Eq, Serialize, Deserialize))]
pub struct Flag(FlagKind);

impl From<i64> for Flag {
    fn from(value: i64) -> Self {
        Flag::new(FlagKind::from(value))
    }
}

impl From<String> for Flag {
    fn from(value: String) -> Self {
        Flag::new(FlagKind::from(value))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum FlagKind {
    Num(i64),
    Str(String),
}

impl fmt::Display for FlagKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FlagKind::Num(value) => write!(f, "{value}"),
            FlagKind::Str(value) => write!(f, "{value}"),
        }
    }
}

impl From<i64> for FlagKind {
    fn from(value: i64) -> Self {
        Self::Num(value)
    }
}

impl From<String> for FlagKind {
    fn from(value: String) -> Self {
        Self::Str(value)
    }
}
