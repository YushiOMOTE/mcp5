use crate::{
    components::{Position, Size},
    grid::{Grid, GRID_SIZE},
    physics::Velocity,
    sprite::Sprite,
};
use macroquad::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Block;

#[derive(Clone, Copy, Debug)]
pub struct Movable;

pub fn create_block(pos: Position) -> (Position, Velocity, Block, Size, Sprite, Grid, Movable) {
    (
        pos,
        Velocity::new(0.0, 0.0),
        Block,
        Size::new(GRID_SIZE, GRID_SIZE),
        Sprite::new(RED),
        Grid,
        Movable,
    )
}

pub fn create_fixed_block(pos: Position) -> (Position, Velocity, Block, Size, Sprite, Grid) {
    (
        pos,
        Velocity::new(0.0, 0.0),
        Block,
        Size::new(GRID_SIZE, GRID_SIZE),
        Sprite::new(GREEN),
        Grid,
    )
}
