use crate::{components, resources};
use specs::prelude::*;

impl<'a> System<'a> for super::Render {
    type SystemData = (
        ReadStorage<'a, components::Position>,
        ReadStorage<'a, components::Geometry>,
        ReadStorage<'a, components::Texture>,
        Read<'a, resources::Camera>,
        Read<'a, resources::Textures>,
        Write<'a, resources::Signals>,
        Write<'a, resources::Keyboard>,
    );

    fn run(
        &mut self,
        (position, geometry, texture, camera, textures, mut signals, mut keyboard): Self::SystemData,
    ) {
        // exit if closed
        if !self.window.is_open() {
            signals.close = true;
            return;
        }

        // set to black
        for i in 0..self.buffer_length {
            self.buffer[i] = 0x000000
        }

        // draw pixels
        // todo: use quadtree to determine if entity is on the screen
        // note: rstar https://crates.io/crates/rstar
        for (pos, geom, tex) in (&position, &geometry, &texture).join() {
            let rx = pos.x + camera.x + self.half_width as f32;
            let ry = pos.y + camera.y + self.half_height as f32;

            // if self.contains(rx, ry) {
            if self.contains(&rx, &ry, &geom) {
                // if contains, draw texture color
                let texture = textures.textures.get(&tex.0).unwrap();

                // for each pixel in geometry
                for oy in geom.min_y..geom.max_y + 1 {
                    for ox in geom.min_x..geom.max_x + 1 {
                        // offset
                        let x = rx + ox as f32;
                        let y = ry + oy as f32;

                        self.draw(x, y, texture.get(ox, oy))
                    }
                }
            }
        }

        // update keyboard
        keyboard.pressed.clear();
        for key in self.window.get_keys().unwrap().iter() {
            keyboard.pressed.insert(*key);
        }

        // render
        self.window
            .update_with_buffer(&self.buffer, self.width, self.height)
            .unwrap();
    }
}
