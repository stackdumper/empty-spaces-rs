use crate::{components, resources};
use image::{GenericImage, GenericImageView, RgbaImage};
use specs::prelude::*;

pub struct Render;

impl<'a> System<'a> for Render {
    type SystemData = (
        ReadStorage<'a, components::Position>,
        ReadStorage<'a, components::Texture>,
        Read<'a, resources::SDL>,
        Read<'a, resources::Assets>,
    );

    fn run(&mut self, (positions, textures, sdl, assets): Self::SystemData) {
        let mut pixels = sdl.pixels.lock().unwrap();
        let mut image = RgbaImage::new(sdl.width, sdl.height);

        // render entities
        for (position, texture) in (&positions, &textures).join() {
            let texture = assets.textures.get(&texture.name).unwrap();

            if position.x < 0.0
                || position.x as u32 + texture.width() > sdl.width
                || position.y < 0.0
                || position.y as u32 + texture.height() > sdl.height
            {
                continue;
            }

            image
                .copy_from(texture, position.x as u32, position.y as u32)
                .unwrap();
        }

        let frame = pixels.get_frame();
        for (i, e) in image.into_vec().iter().enumerate() {
            frame[i] = *e;
        }

        pixels.render().unwrap();
    }
}
