use crate::{components, resources};
use rstar::AABB;
use specs::prelude::*;

impl<'a> System<'a> for super::Render {
    type SystemData = (
        ReadStorage<'a, components::Position>,
        ReadStorage<'a, components::Geometry>,
        ReadStorage<'a, components::Texture>,
        Read<'a, resources::Camera>,
        Read<'a, resources::Textures>,
        Read<'a, resources::Tree>,
        Read<'a, resources::Map>,
        Write<'a, resources::Signals>,
        Write<'a, resources::Keyboard>,
    );

    fn run(
        &mut self,
        (position, geometry, texture, camera, textures, tree, map, mut signals, mut keyboard): Self::SystemData,
    ) {
        // exit if closed
        if !self.window.is_open() {
            signals.close = true;
            return;
        }

        // set to black
        self.clear(0x000000);

        // calculate camera offset
        let cx = camera.x + self.half_width as f32;
        let cy = camera.y + self.half_height as f32;

        // get screen aabb
        let screen = self.get_aabb(&camera);

        // render map
        let submap = map.area(
            screen.lower().1,
            screen.lower().2,
            screen.upper().1,
            screen.upper().2,
        );

        for x in 0..submap.width {
            for y in 0..submap.height {
                let tile = submap.get_texture(x, y);
                let texture = textures.textures.get(&(tile as u32)).unwrap();
            }
        }

        // for tile in submap.textures {

        // }

        // render entities
        for item in tree.tree.locate_in_envelope(&screen) {
            let entity = item.0.unwrap();

            let pos = position.get(entity).unwrap();
            let geom = geometry.get(entity).unwrap();
            let tex = texture.get(entity).unwrap();

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

impl super::Render {
    fn get_aabb<'a>(&self, camera: &Read<'a, resources::Camera>) -> AABB<resources::TreeItem> {
        AABB::from_corners(
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
        )
    }
}
