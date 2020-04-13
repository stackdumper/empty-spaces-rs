use specs::prelude::*;

#[derive(Debug, Default, Copy, Clone)]
pub struct Geometry {
    pub min_x: i16,
    pub min_y: i16,

    pub max_x: i16,
    pub max_y: i16,
}

impl Component for Geometry {
    type Storage = DenseVecStorage<Self>;
}

impl Geometry {
    pub fn square(width: u16, height: u16) -> Self {
        Self {
            min_x: -((width - 1) as i16) / 2,
            min_y: -((height - 1) as i16) / 2,

            max_x: ((width - 1) as i16) / 2,
            max_y: ((height - 1) as i16) / 2,
        }
    }
}
