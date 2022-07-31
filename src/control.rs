use legion::{
    systems::{Builder, CommandBuffer},
    *,
};
use macroquad::prelude::*;

use crate::{
    components::{Direction, Position},
    grid::GRID_SIZE,
    keymap,
    player::create_attack,
};

const RUN_SPEED: f32 = GRID_SIZE * 8.0;
const WALK_SPEED: f32 = GRID_SIZE * 6.0;

#[derive(Debug)]
pub struct Control;

#[system(for_each)]
fn control(
    pos: &mut Position,
    dir: Option<&mut Direction>,
    _: &Control,
    command_buffer: &mut CommandBuffer,
) {
    let step = if is_key_down(keymap::RUN) {
        RUN_SPEED
    } else {
        WALK_SPEED
    };

    if let Some(dir) = dir {
        if is_key_down(keymap::MOVE_DOWN) {
            dir.down();
        }
        if is_key_down(keymap::MOVE_UP) {
            dir.up();
        }
        if is_key_down(keymap::MOVE_LEFT) {
            dir.left();
        }
        if is_key_down(keymap::MOVE_RIGHT) {
            dir.right();
        }
        if is_key_pressed(keymap::ATTACK) {
            command_buffer.push(create_attack(*pos, *dir));
        }
    }

    if is_key_down(keymap::MOVE_DOWN) {
        pos.y += step * get_frame_time();
    }
    if is_key_down(keymap::MOVE_UP) {
        pos.y -= step * get_frame_time();
    }
    if is_key_down(keymap::MOVE_LEFT) {
        pos.x -= step * get_frame_time();
    }
    if is_key_down(keymap::MOVE_RIGHT) {
        pos.x += step * get_frame_time();
    }
}

pub fn setup_systems(builder: &mut Builder) -> &mut Builder {
    builder.add_system(control_system())
}
