use crate::{components, resources};
use specs::prelude::*;

pub struct Position;

impl<'a> System<'a> for Position {
    type SystemData = (
        WriteStorage<'a, components::Position>,
        ReadStorage<'a, components::Velocity>,
        Read<'a, resources::Map>,
    );

    fn run(&mut self, (mut positions, velocities, map): Self::SystemData) {
        for (mut pos, vel) in (&mut positions, &velocities).join() {
            // add precise position
            pos.px = (pos.px as i32 + vel.px) as u32;
            pos.py = (pos.py as i32 + vel.py) as u32;

            // calcualate tile position
            pos.x = pos.px / 10;
            pos.y = pos.py / 10;

            // fit to bounds
            if pos.x >= map.width {
                pos.x = map.width - 1;
            }
            if pos.y >= map.height {
                pos.y = map.height - 1;
            }
        }
    }
}
