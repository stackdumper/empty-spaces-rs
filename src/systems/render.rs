use crate::{components, resources};
use minifb::{Scale, Window, WindowOptions};
use specs::prelude::*;

pub struct Render {
    pub window: Window,
    pub buffer: Vec<u32>,
    pub width: usize,
    pub height: usize,
    pub half_width: usize,
    pub half_height: usize,
}

impl Render {
    pub fn new(title: &str, width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            half_width: width / 2,
            half_height: height / 2,
            buffer: vec![0; width * height],
            window: Window::new(
                title,
                width,
                height,
                WindowOptions {
                    scale: Scale::X1,
                    ..WindowOptions::default()
                },
            )
            .unwrap(),
        }
    }
}

impl<'a> System<'a> for Render {
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
        self.clear(0x000000);

        // draw pixels
        // todo: use quadtree to determine if entity is on the screen
        // note: rstar https://crates.io/crates/rstar
        for (pos, geom, tex) in (&position, &geometry, &texture).join() {
            let rx = pos.x + camera.x + self.half_width as f32;
            let ry = pos.y + camera.y + self.half_height as f32;

            // if self.contains(rx, ry) {
            if self.containss(&rx, &ry, &geom) {
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

impl Render {
    fn clear(&mut self, color: u32) {
        for elem in self.buffer.iter_mut() {
            *elem = color;
        }
    }

    fn draw(&mut self, x: f32, y: f32, color: u32) {
        if color != 0x000000 {
            let offset = ((y as usize) * self.width) + x as usize;

            if offset < self.buffer.len() {
                self.buffer[offset] = color;
            }
        }
    }

    fn contains(&self, x: f32, y: f32) -> bool {
        x >= 0.0 && y >= 0.0 && x < self.width as f32 && y < self.height as f32
    }

    fn containss(&self, x: &f32, y: &f32, geom: &components::Geometry) -> bool {
        return true
            && x + (geom.min_x as f32) >= 0.0
            && y + (geom.min_y as f32) >= 0.0
            && x + (geom.max_x as f32) < self.width as f32
            && y + (geom.max_y as f32) < self.height as f32;
    }
}
