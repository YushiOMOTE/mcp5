use crate::components::{Position, Size};
use legion::*;
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
pub fn draw_sprites(pos: &Position, size: &Size, sprite: &Sprite) {
    draw_rectangle(pos.x, pos.y, size.x, size.y, sprite.color());
}
