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
pub struct Position(Vec3);

impl Position {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(vec3(x, y, z))
    }
}

#[derive(Clone, Copy, Debug, Deref, DerefMut)]
pub struct Size(Vec3);

impl Size {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self(vec3(x, y, z))
    }
}

pub fn to_rect(pos: Position, size: Size) -> Rect {
    Rect::new(pos.x, pos.y, size.x, size.y)
}

pub fn to_inner_rect(rect: Rect) -> Rect {
    assert!(rect.w > 2.0 && rect.h > 2.0);
    Rect::new(rect.x + 1.0, rect.y + 1.0, rect.w - 2.0, rect.h - 2.0)
}

pub fn center(rect: Rect) -> Vec2 {
    rect.point() + rect.size() / 2.0
}
