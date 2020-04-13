use image::*;
use std::collections::HashMap;

#[derive(Default)]
pub struct Textures {
    pub textures: HashMap<u32, Texture>,
}

impl Textures {
    pub fn new() -> Self {
        let mut textures = HashMap::new();

        textures.insert(1, Self::load("src/assets/grass_1.png"));
        textures.insert(2, Self::load("src/assets/grass_2.png"));
        textures.insert(3, Self::load("src/assets/grass_3.png"));
        textures.insert(4, Self::load("src/assets/grass_big.png"));
        textures.insert(5, Self::load("src/assets/commander.png"));
        textures.insert(6, Self::load("src/assets/engineer.png"));

        Self { textures }
    }

    pub fn load(path: &str) -> Texture {
        // open image

        let img = image::open(path).unwrap();
        let width = img.width() as u16;
        let height = img.height() as u16;

        // get array of u32 colors
        let mut colors = vec![0; width as usize * height as usize];
        for (i, rgb) in img.to_rgba().chunks(4).enumerate() {
            colors[i] = ((rgb[0] as u32) << 16) | ((rgb[1] as u32) << 8) | rgb[2] as u32;
        }

        Texture {
            width,
            height,
            colors,
        }
    }
}

#[derive(Debug, Default)]
pub struct Texture {
    pub colors: Vec<u32>,

    width: u16,
    height: u16,
}

impl Texture {
    // pub fn color(width: u8, height: u8, color: u32) -> Self {
    //     Self {
    //         colors: vec![color; (width * height) as usize],
    //         width,
    //         height,
    //     }
    // }

    // pub fn colors(width: u8, height: u8, colors: Vec<u32>) -> Self {
    //     Self {
    //         colors,
    //         width,
    //         height,
    //     }
    // }

    pub fn get(&self, x: i16, y: i16) -> u32 {
        // relative positions
        let rx = (x + (self.width / 2) as i16) as u16;
        let ry = (y + (self.height / 2) as i16) as u16;

        // offset
        let offset = (ry * self.width + rx) as usize;

        return self.colors[offset];
    }
}
