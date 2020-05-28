#[derive(Debug, Default)]
pub struct Map {
    pub width: u32,
    pub height: u32,
}

impl Map {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}
