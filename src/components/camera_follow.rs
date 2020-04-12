use specs::prelude::*;

#[derive(Debug, Default, Copy, Clone)]
pub struct CameraFollow;

impl Component for CameraFollow {
    type Storage = NullStorage<Self>;
}
