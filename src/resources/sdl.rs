use beryllium::*;
use std::sync::{Arc, Mutex};

pub struct SDL {
    pub sdl: Arc<Mutex<beryllium::SDL>>,
    pub window: Arc<Mutex<beryllium::RawWindow>>,
}

impl Default for SDL {
    fn default() -> Self {
        let sdl = beryllium::SDL::init(InitFlags::default()).unwrap();
        let window = sdl
            .create_raw_window("Hello Pixels", WindowPosition::Centered, 300, 200, 0)
            .unwrap();

        Self {
            sdl: Arc::new(Mutex::new(sdl)),
            window: Arc::new(Mutex::new(window)),
        }
    }
}

unsafe impl Send for SDL {}
unsafe impl Sync for SDL {}
