use super::components::Direction;

#[derive(Debug, derive_more::IntoIterator)]
pub struct Day01Input {
    #[into_iterator(owned, ref)]
    directions: Vec<Direction>,
}

impl Day01Input {
    pub(super) fn new(directions: Vec<Direction>) -> Self {
        Self { directions }
    }
}
