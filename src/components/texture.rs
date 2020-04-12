use specs::prelude::*;

#[derive(Debug, Default)]
pub struct Texture(pub u32);

impl Component for Texture {
    type Storage = DenseVecStorage<Self>;
}
