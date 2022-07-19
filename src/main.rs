use derive_deref::{Deref, DerefMut};
use legion::*;
use legion::world::SubWorld;
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

#[derive(Clone, Copy, Debug)]
pub struct Block;

#[derive(Clone, Copy, Debug)]
pub struct Player;

impl Player {
    fn step(&self) -> f32 {
        5.0
    }
}

fn create_player(pos: Position) -> (Position, Direction, Velocity, Player, Size, Sprite) {
    (pos, Direction::Down, Velocity::new(0.0, 0.0), Player, Size::new(40.0, 40.0), Sprite::new(BLUE))
}

fn create_block(pos: Position) -> (Position, Velocity, Block, Size, Sprite) {
    (pos, Velocity::new(0.0, 0.0), Block, Size::new(40.0, 40.0), Sprite::new(RED))
}

#[system(for_each)]
fn update_positions(pos: &mut Position, vel: &Velocity) {
    pos.x += vel.x * get_frame_time();
    pos.y += vel.y * get_frame_time();
}

#[system(for_each)]
fn control_player(pos: &mut Position, dir: &mut Direction, player: &Player) {
    let step = player.step();

    if is_key_down(KeyCode::Down) {
        pos.y += step;
        dir.down();
    }
    if is_key_down(KeyCode::Up) {
        pos.y -= step;
        dir.up();
    }
    if is_key_down(KeyCode::Left) {
        pos.x -= step;
        dir.left();
    }
    if is_key_down(KeyCode::Right) {
        pos.x += step;
        dir.right();
    }
}

#[system]
fn player_block_collision(world: &mut SubWorld, players: &mut Query<(&mut Position, &Size, &Player)>, blocks: &mut Query<(&Position, &Size, &Block)>) {
    let blocks: Vec<_> = blocks.iter(world).map(|(pos, size, _)| to_rect(*pos, *size)).collect();

    for (pos, size, _) in players.iter_mut(world) {
        for block_rect in &blocks {
            let player_rect = to_rect(*pos, *size);
            let margin = Vec2::new(player_rect.w * 0.45, player_rect.h * 0.45);
            adjust_player_rect(pos, &player_rect, block_rect, margin);
        }
    }
}

fn adjust_player_rect(pos: &mut Position, player_rect: &Rect, block_rect: &Rect, margin: Vec2) {
    // collision detection with smaller rectangle otherwise player gets stuck for excessive collision due to rounding errors
    if !to_inner_rect(*player_rect).overlaps(&to_inner_rect(*block_rect)) {
        return;
    }

    let overlap = match player_rect.intersect(*block_rect) {
        Some(rect) => rect,
        None => return
    };

    let player_center = center(*player_rect);
    let touch_up = overlap.y <= player_center.y;
    let touch_right = overlap.x <= player_center.x;

    if overlap.w < overlap.h {
        if touch_right {
            pos.x += overlap.w;
        } else {
            pos.x -= overlap.w;
        }
        assert!(margin.y < player_rect.h / 2.0);
        if overlap.h <= margin.y {
            if touch_up {
                pos.y += overlap.h;
            } else {
                pos.y -= overlap.h;
            }
        } else if overlap.h >= player_rect.h - margin.y {
            if player_rect.y <= block_rect.y {
                pos.y += player_rect.h - overlap.h;
            } else {
                pos.y -= player_rect.h - overlap.h;
            }
        }
    } else {
        if touch_up {
            pos.y += overlap.h;
        } else {
            pos.y -= overlap.h;
        }
        assert!(margin.x < player_rect.w / 2.0);
        if overlap.w <= margin.x {
            if touch_right {
                pos.x += overlap.w;
            } else {
                pos.x -= overlap.w;
            }
        } else if overlap.w >= player_rect.w - margin.x {
            if player_rect.x <= block_rect.x {
                pos.x += player_rect.w - overlap.w;
            } else {
                pos.x -= player_rect.w - overlap.w;
            }
        }
    }
}

#[system(for_each)]
fn draw_sprites(pos: &Position, size: &Size, sprite: &Sprite) {
    draw_rectangle(pos.x, pos.y, size.x, size.y, sprite.color());
}

#[macroquad::main("gf")]
async fn main() {
    let mut world = World::default();
    let mut resources = Resources::default();

    world.push(create_player(Position::new(120.0, 120.0)));
    world.extend(vec![
        create_block(Position::new(0.0, 0.0)),
        create_block(Position::new(0.0, 40.0)),
        create_block(Position::new(0.0, 80.0)),
        create_block(Position::new(0.0, 120.0)),
        create_block(Position::new(80.0, 80.0)),
        create_block(Position::new(160.0, 80.0)),
        create_block(Position::new(240.0, 80.0)),
    ]);

    let mut schedule = Schedule::builder()
        .add_system(update_positions_system())
        .add_system(draw_sprites_system())
        .add_system(control_player_system())
        .add_system(player_block_collision_system())
        .build();

    while !is_key_down(KeyCode::Escape) {
        clear_background(WHITE);

        schedule.execute(&mut world, &mut resources);

        next_frame().await
    }
}
