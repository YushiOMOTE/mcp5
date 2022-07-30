use crate::{
    components::{Position, Size},
    map::Terrain,
};
use legion::{systems::Builder, world::SubWorld, *};
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

#[system]
#[read_component(Position)]
#[read_component(Size)]
#[read_component(Sprite)]
#[read_component(Terrain)]
fn draw_sprites(world: &mut SubWorld) {
    let mut background = <(&Position, &Size, &Sprite, &Terrain)>::query();
    let mut foreground = <(&Position, &Size, &Sprite)>::query().filter(!component::<Terrain>());

    for (pos, size, sprite, _) in background.iter(world) {
        draw_rectangle(pos.x, pos.y, size.x, size.y, sprite.color());
    }
    for (pos, size, sprite) in foreground.iter(world) {
        draw_rectangle(pos.x, pos.y, size.x, size.y, sprite.color());
    }
}

pub fn setup_systems(builder: &mut Builder) -> &mut Builder {
    builder.add_system(draw_sprites_system())
}
