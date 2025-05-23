use super::components::NextMove;

#[derive(Debug, Clone, derive_more::IntoIterator)]
pub struct Input(#[into_iterator(owned, ref)] Vec<NextMove>);

impl Input {
    pub(super) fn new(moves: Vec<NextMove>) -> Self {
        Self(moves)
    }
}
