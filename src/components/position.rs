use specs::prelude::*;

#[derive(Debug, Default)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Component for Position {
    type Storage = DenseVecStorage<Self>;
}

/// represents entity position
impl Position {
    /// creates a new Position instance
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}
