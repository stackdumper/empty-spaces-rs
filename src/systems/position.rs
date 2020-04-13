use crate::{components, resources};
use specs::prelude::*;

pub struct Position;

impl<'a> System<'a> for Position {
    type SystemData = (
        WriteStorage<'a, components::Position>,
        ReadStorage<'a, components::Velocity>,
        Read<'a, resources::Clock>,
    );

    fn run(&mut self, (mut pos, vel, clock): Self::SystemData) {
        println!("{}", clock.dt);

        for (pos, vel) in (&mut pos, &vel).join() {
            pos.x += vel.x * clock.dt;
            pos.y += vel.y * clock.dt;
        }
    }
}
