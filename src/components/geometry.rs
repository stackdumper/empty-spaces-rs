use specs::prelude::*;

#[derive(Debug, Default, Copy, Clone)]
pub struct Geometry {
    pub min_x: i8,
    pub min_y: i8,

    pub max_x: i8,
    pub max_y: i8,
}

impl Component for Geometry {
    type Storage = DenseVecStorage<Self>;
}

impl Geometry {
    pub fn square(width: u8, height: u8) -> Self {
        Self {
            min_x: -((width - 1) as i8) / 2,
            min_y: -((height - 1) as i8) / 2,

            max_x: ((width - 1) as i8) / 2,
            max_y: ((height - 1) as i8) / 2,
        }
    }
}
