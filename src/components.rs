use derive_deref::{Deref, DerefMut};
use macroquad::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn up(&mut self) {
        *self = Self::Up;
    }

    pub fn down(&mut self) {
        *self = Self::Down;
    }

    pub fn left(&mut self) {
        *self = Self::Left;
    }

    pub fn right(&mut self) {
        *self = Self::Right;
    }
}

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
