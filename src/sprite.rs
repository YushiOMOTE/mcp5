use crate::components::{Position, Size};
use legion::{systems::Builder, *};
use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub struct Sprite {
    color: Color,
}

impl Sprite {
    pub fn new(color: Color) -> Self {
        Self { color }
    }

    pub fn color(&self) -> Color {
        self.color
    }
}

#[system(for_each)]
fn draw_sprites(pos: &Position, size: &Size, sprite: &Sprite) {
    draw_rectangle(pos.x, pos.y, size.x, size.y, sprite.color());
}

pub fn setup_systems(builder: &mut Builder) -> &mut Builder {
    builder.add_system(draw_sprites_system())
}
