use legion::*;
use macroquad::prelude::*;
use rapier3d::prelude::*;

use crate::keymap;

const RUN_SPEED: f32 = 80.0;
const WALK_SPEED: f32 = 40.0;

#[derive(Debug)]
pub struct Control;

#[system(for_each)]
pub fn control(
    _: &Control,
    rigid_body_handle: &RigidBodyHandle,
    #[resource] rigid_body_set: &mut RigidBodySet,
) {
    let step = if is_key_down(keymap::RUN) {
        RUN_SPEED
    } else {
        WALK_SPEED
    };

    if is_key_down(keymap::JUMP) {
        jump(
            rigid_body_set,
            rigid_body_handle,
            vector![0.0, 0.0, 10000.0],
        );
    }

    let pos_y = if is_key_down(keymap::MOVE_POS_Y) {
        step
    } else {
        0.0
    };
    let neg_y = if is_key_down(keymap::MOVE_NEG_Y) {
        step
    } else {
        0.0
    };

    let pos_x = if is_key_down(keymap::MOVE_POS_X) {
        step
    } else {
        0.0
    };
    let neg_x = if is_key_down(keymap::MOVE_NEG_X) {
        step
    } else {
        0.0
    };

    set_velocity(
        rigid_body_set,
        rigid_body_handle,
        vector![pos_x - neg_x, pos_y - neg_y, 0.0],
    );
}

fn set_velocity(
    rigid_body_set: &mut RigidBodySet,
    rigid_body_handle: &RigidBodyHandle,
    vel: Vector<f32>,
) {
    if let Some(body) = rigid_body_set.get_mut(*rigid_body_handle) {
        body.set_linvel(vector![vel.x, vel.y, body.linvel().z], true);
    }
}

fn jump(
    rigid_body_set: &mut RigidBodySet,
    rigid_body_handle: &RigidBodyHandle,
    impulse: Vector<f32>,
) {
    if let Some(body) = rigid_body_set.get_mut(*rigid_body_handle) {
        if is_on_the_ground(body) {
            body.apply_impulse(impulse, true);
        }
    }
}

fn is_on_the_ground(body: &RigidBody) -> bool {
    let gravity = vector![0.0, 0.0, -9.81];
    let energy1 = body.gravitational_potential_energy(0.2, gravity);
    let energy2 = body.gravitational_potential_energy(0.5, gravity);
    energy1 == energy2
}
