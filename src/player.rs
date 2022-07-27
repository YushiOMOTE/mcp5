use crate::{
    camera::Camera,
    components::{Direction, Position, Size},
    control::Control,
    grid::GRID_SIZE,
    physics::Velocity,
    sprite::Sprite,
};
use macroquad::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Player;

#[derive(Clone, Copy, Debug)]
pub struct PlayerPart;

pub fn create_player(
    pos: Position,
) -> (
    Position,
    Direction,
    Velocity,
    Player,
    PlayerPart,
    Size,
    Sprite,
    Camera,
    Control,
) {
    (
        pos,
        Direction::Down,
        Velocity::new(0.0, 0.0),
        Player,
        PlayerPart,
        Size::new(GRID_SIZE, GRID_SIZE),
        Sprite::new(BLUE),
        Camera,
        Control,
    )
}
