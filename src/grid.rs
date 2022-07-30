use crate::components::Position;
use legion::{systems::Builder, *};
use macroquad::prelude::*;

pub const GRID_SIZE: f32 = 40.0;

#[derive(Clone, Debug)]
pub struct Grid;

#[system(for_each)]
fn grid(pos: &mut Position, _: &Grid) {
    pos.x = (pos.x / GRID_SIZE).round() * GRID_SIZE;
    pos.y = (pos.y / GRID_SIZE).round() * GRID_SIZE;
}

pub fn setup_systems(builder: &mut Builder) -> &mut Builder {
    builder.add_system(grid_system())
}
