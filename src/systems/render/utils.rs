use crate::components;

impl super::Render {
    pub fn clear(&mut self, color: u32) {
        for i in 0..self.buffer_length {
            self.buffer[i] = color;
        }
    }

    pub fn draw(&mut self, x: f32, y: f32, color: u32) {
        if x >= 0.0 && y >= 0.0 && x < self.width as f32 && y < self.height as f32 {
            let offset = ((y as usize) * self.width) + x as usize;

            if offset < self.buffer_length && self.buffer[offset] == 0x000000 {
                self.buffer[offset] = color;
            }
        }
    }

    pub fn contains(&self, x: &f32, y: &f32, geom: &components::Geometry) -> bool {
        return x + (geom.max_x as f32) >= 0.0
            && y + (geom.max_y as f32) >= 0.0
            && x + (geom.min_x as f32) < self.width as f32
            && y + (geom.min_y as f32) < self.height as f32;
    }
}
