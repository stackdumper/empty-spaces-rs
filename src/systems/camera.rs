use crate::{components, resources};
use specs::prelude::*;

pub struct Camera;

impl<'a> System<'a> for Camera {
    type SystemData = (
        Write<'a, resources::Camera>,
        ReadStorage<'a, components::CameraFollow>,
        ReadStorage<'a, components::Position>,
    );

    fn run(&mut self, (mut camera, follow, position): Self::SystemData) {
        for (_follow, position) in (&follow, &position).join() {
            camera.x = position.x;
            camera.y = position.y;
        }
    }
}
