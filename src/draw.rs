use crate::{
    components::{Position, Size},
    terrain::Terrain,
};
use legion::{world::SubWorld, *};
use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub struct Sprite {
    color: Color,
    texture: Option<Texture2D>,
}

impl Sprite {
    pub fn new(color: Color, texture: Option<Texture2D>) -> Self {
        Self {
            color,
            texture: texture,
        }
    }

    pub fn plain(color: Color) -> Self {
        Self {
            color,
            texture: None,
        }
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn texture(&self) -> Option<Texture2D> {
        self.texture
    }
}

#[system]
#[read_component(Position)]
#[read_component(Size)]
#[read_component(Sprite)]
#[read_component(Terrain)]
pub fn draw(world: &mut SubWorld) {
    let mut items = <(&Position, &Size, &Sprite)>::query();
    for (pos, size, sprite) in items.iter(world) {
        draw_cube(
            vec3(pos.x, pos.y, pos.z),
            vec3(size.x, size.y, size.z),
            sprite.texture(),
            sprite.color(),
        );
    }
}
