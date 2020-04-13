use specs::prelude::*;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

impl Component for Position {
    type Storage = FlaggedStorage<Self, DenseVecStorage<Self>>;
}
