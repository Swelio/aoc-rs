#[derive(Debug, Clone, Copy)]
pub struct Cuboid {
    pub(super) length: u32,
    pub(super) width: u32,
    pub(super) height: u32,
}

impl Cuboid {
    pub(super) fn new(length: u32, width: u32, height: u32) -> Self {
        Self {
            length,
            width,
            height,
        }
    }
}
