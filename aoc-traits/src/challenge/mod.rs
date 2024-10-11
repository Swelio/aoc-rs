pub use identity::{identity_raw_parser, Day, Identity, IdentityError, Part, Year};
pub use input::{ChallengeInput, InputName};
pub use model::Challenge;
pub use solution::{Flag, Solution};

mod identity;
mod input;
mod model;
mod solution;
