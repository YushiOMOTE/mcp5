use derive_deref::{Deref, DerefMut};
use macroquad::color::Color;
use macroquad::math::Rect;

use crate::{f32, Vec2, vec2};

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
        *self = Self::Up;
    }

    pub fn left(&mut self) {
        *self = Self::Left;
    }

    pub fn right(&mut self) {
        *self = Self::Right;
    }
}

#[derive(Clone, Copy, Debug, Deref, DerefMut)]
pub struct Position(Vec2);

impl Position {
    pub fn new(x: f32, y: f32) -> Self {
        Self(vec2(x, y))
    }
}

#[derive(Clone, Copy, Debug, Deref, DerefMut)]
pub struct Velocity(Vec2);

impl Velocity {
    pub fn new(x: f32, y: f32) -> Self {
        Self(vec2(x, y))
    }
}

#[derive(Clone, Copy, Debug, Deref, DerefMut)]
pub struct Size(Vec2);

impl Size {
    pub fn new(x: f32, y: f32) -> Self {
        Self(vec2(x, y))
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

#[derive(Clone, Debug)]
pub struct Sprite {
    color: Color,
}

impl Sprite {
    pub fn new(color: Color) -> Self {
        Self { color }
    }

    pub fn color(&self) -> Color {
        self.color
    }
}
