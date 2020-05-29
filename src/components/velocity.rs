use specs::prelude::*;

/// represents entity velocity
#[derive(Debug, Default)]
pub struct Velocity {
    pub px: i32,
    pub py: i32,
}

impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}

impl Velocity {
    pub fn new(px: i32, py: i32) -> Self {
        Self { px, py }
    }
}
