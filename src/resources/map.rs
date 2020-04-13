
#[derive(Copy, Clone)]
pub enum Texture {
    Grass = 1,
    Block = 2,
}

#[derive(Copy, Clone)]
pub enum Traverse {
    Bloc = 0,
    Free = 1,
}

pub struct Map {
    pub width: usize,
    pub height: usize,

    pub tile_size: usize,

    pub textures: Vec<Texture>,
    pub traverse: Vec<Traverse>,
}

impl Map {
    pub fn area(&self, min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> Self {
        let min_x_tile = min_x as usize / self.tile_size;
        let min_y_tile = min_y as usize / self.tile_size;
        let max_x_tile = max_x as usize / self.tile_size;
        let max_y_tile = max_y as usize / self.tile_size;

        let start = (min_y_tile * self.width) + min_x_tile as usize;
        let end = (max_y_tile * self.width) + max_x_tile as usize;

        Self {
            width: max_x_tile - min_x_tile,
            height: max_y_tile - min_y_tile,

            tile_size: self.tile_size,

            textures: self.textures[start..end].to_vec(),
            traverse: self.traverse[start..end].to_vec(),
        }
    }

    pub fn get_texture(&self, x: usize, y: usize) -> Texture {
        let offset = (x * self.width) + x as usize;

        return self.textures[offset];
    }
}

impl Default for Map {
    fn default() -> Self {
        Self {
            width: 6,
            height: 6,

            tile_size: 8,

            textures: vec![
                Texture::Block, Texture::Block, Texture::Block, Texture::Block, Texture::Block, Texture::Block, //
                Texture::Block, Texture::Grass, Texture::Grass, Texture::Grass, Texture::Grass, Texture::Block, //
                Texture::Block, Texture::Grass, Texture::Grass, Texture::Grass, Texture::Grass, Texture::Block, //
                Texture::Block, Texture::Grass, Texture::Grass, Texture::Grass, Texture::Grass, Texture::Block, //
                Texture::Block, Texture::Grass, Texture::Grass, Texture::Grass, Texture::Grass, Texture::Block, //
                Texture::Block, Texture::Block, Texture::Block, Texture::Block, Texture::Block, Texture::Block, //
            ],

            traverse: vec![
                Traverse::Bloc, Traverse::Bloc, Traverse::Bloc, Traverse::Bloc, Traverse::Bloc, Traverse::Bloc, //
                Traverse::Bloc, Traverse::Free, Traverse::Free, Traverse::Free, Traverse::Free, Traverse::Bloc, //
                Traverse::Bloc, Traverse::Free, Traverse::Free, Traverse::Free, Traverse::Free, Traverse::Bloc, //
                Traverse::Bloc, Traverse::Free, Traverse::Free, Traverse::Free, Traverse::Free, Traverse::Bloc, //
                Traverse::Bloc, Traverse::Free, Traverse::Free, Traverse::Free, Traverse::Free, Traverse::Bloc, //
                Traverse::Bloc, Traverse::Bloc, Traverse::Bloc, Traverse::Bloc, Traverse::Bloc, Traverse::Bloc, //
            ],
        }
    }
}
