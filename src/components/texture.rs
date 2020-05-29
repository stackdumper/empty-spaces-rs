use specs::prelude::*;

/// represents entity texture
#[derive(Debug, Default)]
pub struct Texture {
    pub name: String,
}

impl Component for Texture {
    type Storage = DenseVecStorage<Self>;
}

impl Texture {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
