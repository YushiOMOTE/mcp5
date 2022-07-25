use crate::{
    camera::Camera,
    components::{Direction, Position, Size},
    grid::GRID_SIZE,
    physics::Velocity,
    sprite::Sprite,
};
use legion::*;
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
    )
}

#[system(for_each)]
pub fn control_player(pos: &mut Position, dir: Option<&mut Direction>, _: &PlayerPart) {
    let step = if is_key_down(KeyCode::X) {
        400.0
    } else {
        200.0
    };

    if let Some(dir) = dir {
        if is_key_down(KeyCode::Down) {
            dir.down();
        }
        if is_key_down(KeyCode::Up) {
            dir.up();
        }
        if is_key_down(KeyCode::Left) {
            dir.left();
        }
        if is_key_down(KeyCode::Right) {
            dir.right();
        }
    }

    if is_key_down(KeyCode::Down) {
        pos.y += step * get_frame_time();
    }
    if is_key_down(KeyCode::Up) {
        pos.y -= step * get_frame_time();
    }
    if is_key_down(KeyCode::Left) {
        pos.x -= step * get_frame_time();
    }
    if is_key_down(KeyCode::Right) {
        pos.x += step * get_frame_time();
    }
}
