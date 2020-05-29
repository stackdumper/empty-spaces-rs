use crate::components;
use specs::prelude::*;

pub struct Position;

impl<'a> System<'a> for Position {
    type SystemData = (
        WriteStorage<'a, components::Position>,
        ReadStorage<'a, components::Velocity>,
    );

    fn run(&mut self, (mut positions, velocities): Self::SystemData) {
        for (mut pos, vel) in (&mut positions, &velocities).join() {
            // add precise position
            pos.x += vel.x;
            pos.y += vel.y;
        }
    }
}
