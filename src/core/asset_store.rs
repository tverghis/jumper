use ggez::graphics::{Font, Image};
use std::collections::HashMap;

#[derive(Default)]
pub struct AssetStore {
    fonts: HashMap<&'static str, Font>,
    images: HashMap<&'static str, Image>,
}

impl AssetStore {
    pub fn new() -> AssetStore {
        AssetStore {
            ..Default::default()
        }
    }

    pub fn store_font(&mut self, descriptor: &'static str, font: Font) {
        self.fonts.insert(descriptor, font);
    }

    pub fn get_font(&self, descriptor: &str) -> Option<&Font> {
        self.fonts.get(descriptor)
    }

    pub fn store_image(&mut self, descriptor: &'static str, image: Image) {
        self.images.insert(descriptor, image);
    }

    pub fn get_image(&self, descriptor: &'static str) -> Option<&Image> {
        self.images.get(descriptor)
    }
}
