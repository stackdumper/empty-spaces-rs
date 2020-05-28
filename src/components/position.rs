use specs::prelude::*;

#[derive(Debug, Default)]
pub struct Position {
    // tile position
    // for rendering and collisions
    // x == px * 10
    pub x: u32,
    pub y: u32,

    // precise subtile position
    // for movement and other logic
    // px == x / 10
    pub px: u32,
    pub py: u32,
}

impl Component for Position {
    type Storage = DenseVecStorage<Self>;
}

/// represents entity position
impl Position {
    /// creates new Position from tile coordinates
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            x,
            y,

            px: x * 10,
            py: y * 10,
        }
    }

    /// updates precise position and automatically recomputes tile position
    pub fn set(&mut self, px: u32, py: u32) {
        self.px = px;
        self.py = py;

        self.x = px / 10;
        self.y = py / 10;
    }
}
