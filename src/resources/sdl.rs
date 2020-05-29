use beryllium::*;
use pixels::wgpu::Surface;
use pixels::{Pixels, SurfaceTexture};
use std::sync::{Arc, Mutex};

pub struct SDL {
    pub width: u32,
    pub height: u32,
    pub sdl: Arc<Mutex<beryllium::SDL>>,
    pub window: Arc<Mutex<beryllium::RawWindow>>,
    pub pixels: Arc<Mutex<Pixels>>,
}

impl SDL {
    pub fn new(width: u32, height: u32) -> Self {
        let sdl = beryllium::SDL::init(InitFlags::default()).unwrap();
        let window = sdl
            .create_raw_window("Empty Spaces", WindowPosition::Centered, width, height, 0)
            .unwrap();

        let surface = Surface::create(&window);
        let surface_texture = SurfaceTexture::new(width, height, surface);
        let pixels = Pixels::new(width, height, surface_texture).unwrap();

        Self {
            width: width,
            height: height,
            sdl: Arc::new(Mutex::new(sdl)),
            window: Arc::new(Mutex::new(window)),
            pixels: Arc::new(Mutex::new(pixels)),
        }
    }
}

impl Default for SDL {
    fn default() -> Self {
        Self::new(300, 200)
    }
}

unsafe impl Send for SDL {}
unsafe impl Sync for SDL {}
