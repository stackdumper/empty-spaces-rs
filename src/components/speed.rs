use specs::prelude::*;

#[derive(Debug, Default)]
pub struct Speed {
    pub x: f32,
    pub y: f32,
}

impl Component for Speed {
    type Storage = DenseVecStorage<Self>;
}
