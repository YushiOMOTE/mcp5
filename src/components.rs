use derive_deref::{Deref, DerefMut};
use macroquad::prelude::*;

#[derive(Clone, Copy, Debug, Deref, DerefMut)]
pub struct Position(pub Vec3);

impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(vec3(x, y, z))
    }
}

#[derive(Clone, Copy, Debug, Deref, DerefMut)]
pub struct Size(pub Vec3);

impl Size {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(vec3(x, y, z))
    }
}
