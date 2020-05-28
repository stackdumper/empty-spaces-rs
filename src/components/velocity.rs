use specs::prelude::*;

/// represents entiy position
#[derive(Debug, Default)]
pub struct Velocity {
    pub px: u32,
    pub py: u32,
}

impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}

impl Velocity {
    pub fn new(px: u32, py: u32) -> Self {
        Self { px, py }
    }
}
