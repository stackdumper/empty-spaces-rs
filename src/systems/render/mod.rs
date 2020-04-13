use minifb::{Scale, Window, WindowOptions};

mod render;
mod system;

pub struct Render {
    pub window: Window,
    pub buffer: Vec<u32>,
    pub buffer_length: usize,
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
            buffer_length: width * height,
            window: Window::new(
                title,
                width,
                height,
                WindowOptions {
                    scale: Scale::X2,
                    ..WindowOptions::default()
                },
            )
            .unwrap(),
        }
    }
}
