use legion::{systems::CommandBuffer, *};
use macroquad::prelude::*;
use rapier3d::prelude::*;

use crate::{
    components::{Direction, Position},
    grid::GRID_SIZE,
    keymap,
};

const RUN_SPEED: f32 = GRID_SIZE * 8.0;
const WALK_SPEED: f32 = GRID_SIZE * 6.0;

#[derive(Debug)]
pub struct Control;

#[system(for_each)]
pub fn control(
    pos: &Position,
    dir: Option<&mut Direction>,
    _: &Control,
    rigid_body_handle: &RigidBodyHandle,
    command_buffer: &mut CommandBuffer,
    #[resource] rigid_body_set: &mut RigidBodySet,
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
        if is_key_pressed(keymap::ATTACK) {}
    }

    if is_key_down(keymap::JUMP) {
        apply_impulse(
            rigid_body_set,
            rigid_body_handle,
            vector![0.0, 0.0, -1000.0],
        );
    }

    if is_key_down(keymap::MOVE_DOWN) {
        apply_impulse(rigid_body_set, rigid_body_handle, vector![0.0, 500.0, 0.0]);
    }
    if is_key_down(keymap::MOVE_UP) {
        apply_impulse(rigid_body_set, rigid_body_handle, vector![0.0, -500.0, 0.0]);
    }
    if is_key_down(keymap::MOVE_LEFT) {
        apply_impulse(rigid_body_set, rigid_body_handle, vector![-500.0, 0.0, 0.0]);
    }
    if is_key_down(keymap::MOVE_RIGHT) {
        apply_impulse(rigid_body_set, rigid_body_handle, vector![500.0, 0.0, 0.0]);
    }
}

fn apply_impulse(
    rigid_body_set: &mut RigidBodySet,
    rigid_body_handle: &RigidBodyHandle,
    impulse: Vector<f32>,
) {
    if let Some(body) = rigid_body_set.get_mut(*rigid_body_handle) {
        body.reset_forces(true);
        body.apply_impulse(impulse, true);
    }
}
