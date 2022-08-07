use legion::*;
use macroquad::prelude::*;

use crate::components::{Position, Size};

#[derive(Debug)]
pub struct Camera;

#[system(for_each)]
pub fn update_camera(position: &Position, size: &Size, _: &Camera) {
    let target = **position + **size * 0.5;

    set_camera(&Camera3D {
        position: vec3(target.x - 30.0, target.y - 50.0, target.z + 80.0),
        up: vec3(0., 0., 1.),
        target,
        ..Default::default()
    });
}
