use crate::{components, resources};
use minifb::Key;
use specs::prelude::*;

pub struct Controls;

impl<'a> System<'a> for Controls {
    type SystemData = (
        Read<'a, resources::Keyboard>,
        ReadStorage<'a, components::CameraFollow>,
        ReadStorage<'a, components::Speed>,
        WriteStorage<'a, components::Velocity>,
    );

    fn run(&mut self, (keyboard, follow, speed, mut velocity): Self::SystemData) {
        for (_follow, speed, mut velocity) in (&follow, &speed, &mut velocity).join() {
            velocity.x = 0.0;
            velocity.y = 0.0;

            for key in keyboard.pressed.iter() {
                match key {
                    &Key::W => velocity.y = -speed.y,
                    &Key::S => velocity.y = speed.y,
                    &Key::A => velocity.x = -speed.x,
                    &Key::D => velocity.x = speed.x,
                    _ => {}
                }
            }
        }
    }
}
