use image;
use std::collections::HashMap;
use std::fs;

pub struct Assets {
    pub textures: HashMap<String, image::DynamicImage>,
}

impl Assets {
    pub fn new(path: &str) -> Self {
        let mut textures = HashMap::new();

        for path in fs::read_dir(path).unwrap() {
            let path = path.unwrap().path().into_os_string().into_string().unwrap();
            let texture = image::open(path.clone()).unwrap();

            println!("{}", path);

            textures.insert(path.clone(), texture);
        }

        Self { textures }
    }
}

impl Default for Assets {
    fn default() -> Self {
        Self::new("src/assets")
    }
}
