use specs::prelude::*;

/// represents entity velocity
#[derive(Debug, Default)]
pub struct Velocity {
    pub x: f64,
    pub y: f64,
}

impl Component for Velocity {
    type Storage = DenseVecStorage<Self>;
}

impl Velocity {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}
