use crate::{components, resources};
use rstar::AABB;
use specs::prelude::*;
use std::collections::HashSet;

impl<'a> System<'a> for super::Render {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, components::Position>,
        ReadStorage<'a, components::Geometry>,
        ReadStorage<'a, components::Texture>,
        Read<'a, resources::Camera>,
        Read<'a, resources::Textures>,
        Read<'a, resources::Tree>,
        Write<'a, resources::Signals>,
        Write<'a, resources::Keyboard>,
    );

    fn run(
        &mut self,
        (entities, position, geometry, texture, camera, textures, tree, mut signals, mut keyboard): Self::SystemData,
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

        let ent = tree
            .tree
            .locate_in_envelope(&AABB::from_corners(
                resources::TreeItem {
                    0: None,
                    1: -camera.x - self.half_width as f32,
                    2: -camera.y - self.half_height as f32,
                },
                resources::TreeItem {
                    0: None,
                    1: -camera.x + self.half_width as f32,
                    2: -camera.y + self.half_height as f32,
                },
            ))
            .into_iter()
            .map(|i| i.0.unwrap())
            .collect::<HashSet<Entity>>();

        let cx = camera.x + self.half_width as f32;
        let cy = camera.y + self.half_height as f32;

        for (_, pos, geom, tex) in (&entities, &position, &geometry, &texture)
            .join()
            .filter(|(entity, _, __, ___)| ent.contains(entity))
        {
            let rx = pos.x + cx;
            let ry = pos.y + cy;

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
