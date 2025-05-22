#[derive(Debug, Clone, Copy)]
pub struct Cuboid {
    pub(super) length: i32,
    pub(super) width: i32,
    pub(super) height: i32,
}

impl Cuboid {
    pub(super) fn new(length: i32, width: i32, height: i32) -> Self {
        Self {
            length,
            width,
            height,
        }
    }
}
