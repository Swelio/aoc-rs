use aoc_core::views::SolutionView;
use async_trait::async_trait;

use crate::{components::challenge::ChallengeInput, error::WebResult};

#[async_trait]
pub trait SolvingService: Send + Sync + Clone + 'static {
    async fn solve(&self, input: ChallengeInput) -> WebResult<SolutionView>;
}
