use legion::{systems::Builder, *};
use macroquad::prelude::*;

use crate::components::{self, Position, Size};

#[derive(Debug)]
pub struct Camera;

#[system(for_each)]
fn update_camera(position: &Position, size: &Size, _: &Camera) {
    let center = components::center(components::to_rect(*position, *size));

    set_camera(&Camera3D {
        position: vec3(center.x - 100.0, center.y + 200.0, -400.),
        up: vec3(0., 0., -1.),
        target: vec3(center.x, center.y, 0.),
        ..Default::default()
    });
}

pub fn setup_systems(builder: &mut Builder) -> &mut Builder {
    builder.add_system(update_camera_system())
}
