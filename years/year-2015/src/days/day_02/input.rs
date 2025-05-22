use super::components::Cuboid;

#[derive(Debug, derive_more::Constructor, derive_more::IntoIterator)]
pub struct Input {
    #[into_iterator(owned, ref)]
    cuboids: Vec<Cuboid>,
}
