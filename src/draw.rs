use crate::{
    components::{Position, Size},
    grid::GRID_SIZE,
    terrain::Terrain,
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
    let mut items = <(&Position, &Size, &Sprite)>::query();
    for (pos, size, sprite) in items.iter(world) {
        draw_cube(
            vec3(pos.x, pos.y, pos.z),
            vec3(size.x, size.y, GRID_SIZE),
            None,
            sprite.color(),
        );
    }
}

pub fn setup_systems(builder: &mut Builder) -> &mut Builder {
    builder.add_system(draw_sprites_system())
}
