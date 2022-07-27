use legion::*;
use macroquad::prelude::*;

use crate::{
    components::{Direction, Position},
    keymap,
};

#[derive(Debug)]
pub struct Control;

#[system(for_each)]
pub fn control(pos: &mut Position, dir: Option<&mut Direction>, _: &Control) {
    let step = if is_key_down(keymap::RUN) {
        400.0
    } else {
        200.0
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
