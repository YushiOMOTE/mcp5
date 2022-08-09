use legion::Resources;
use macroquad::prelude::*;
use std::collections::HashMap;

pub type TextureId = u64;

#[derive(Debug, Clone)]
pub struct Textures {
    textures: HashMap<TextureId, Texture2D>,
}

impl Textures {
    pub fn new() -> Self {
        Self {
            textures: HashMap::new(),
        }
    }

    pub fn add(&mut self, id: TextureId, texture: Texture2D) {
        self.textures.insert(id, texture);
    }

    pub fn get(&self, id: TextureId) -> Option<Texture2D> {
        self.textures.get(&id).cloned()
    }
}

pub fn setup_resources(resources: &mut Resources) {
    resources.insert(Textures::new());
}
