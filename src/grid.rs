use crate::components::Position;
use legion::*;
use macroquad::prelude::*;

pub const GRID_SIZE: f32 = 40.0;

#[derive(Clone, Debug)]
pub struct Grid;

#[system(for_each)]
pub fn grid(pos: &mut Position, _: &Grid) {
    pos.x = (pos.x / GRID_SIZE).round() * GRID_SIZE;
    pos.y = (pos.y / GRID_SIZE).round() * GRID_SIZE;
}
