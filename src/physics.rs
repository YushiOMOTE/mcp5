use crate::components::Position;
use derive_deref::{Deref, DerefMut};
use legion::{systems::Builder, *};
use macroquad::prelude::*;

#[derive(Clone, Copy, Debug, Deref, DerefMut)]
pub struct Velocity(Vec3);

impl Velocity {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(vec3(x, y, z))
    }
}

#[system(for_each)]
fn update_positions(pos: &mut Position, vel: &Velocity) {
    **pos += **vel * get_frame_time();
}

pub fn setup_systems(builder: &mut Builder) -> &mut Builder {
    builder.add_system(update_positions_system())
}
