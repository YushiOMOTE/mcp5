use crate::{
    components::{Position, Size},
    grid::GRID_SIZE,
    map::Terrain,
};
use legion::{systems::Builder, world::SubWorld, *};
use macroquad::prelude::*;
use std::collections::HashMap;

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

    let mut heights = HashMap::new();

    for (pos, size, sprite, _) in background.iter(world) {
        draw_cube(
            vec3(pos.x, pos.y, pos.z),
            vec3(size.x, size.y, size.z),
            None,
            sprite.color(),
        );

        heights.insert(to_grid_coord(pos), pos.z);
    }

    for (pos, size, sprite) in foreground.iter(world) {
        // TODO: use pos.z
        let z = match heights.get(&to_grid_coord(pos)) {
            Some(z) => z - GRID_SIZE,
            None => 0.0,
        };

        draw_cube(
            vec3(pos.x, pos.y, z),
            vec3(size.x, size.y, GRID_SIZE),
            None,
            sprite.color(),
        );
    }
}

pub fn setup_systems(builder: &mut Builder) -> &mut Builder {
    builder.add_system(draw_sprites_system())
}

fn to_grid_coord(pos: &Position) -> (u64, u64) {
    let grid = GRID_SIZE as u64;
    (pos.x as u64 / grid, pos.y as u64 / grid)
}
