use crate::{
    ai::chase::Chase,
    camera::Camera,
    components::{Direction, Position, Size},
    control::Control,
    grid::GRID_SIZE,
    physics::Velocity,
    sprite::Sprite,
    temporary::Temporary,
};
use legion::Entity;
use macroquad::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Player;

/// Represents a block held by players.
/// `owner` stores the entity of the player who holds this block.
#[derive(Clone, Copy, Debug)]
pub struct PlayerPart {
    owner: Option<Entity>,
}

impl PlayerPart {
    pub fn new(owner: Entity) -> Self {
        Self { owner: Some(owner) }
    }

    pub fn owner(&self) -> Option<Entity> {
        self.owner
    }

    fn empty() -> Self {
        Self { owner: None }
    }
}

pub fn create_player(
    pos: Position,
) -> (
    Position,
    Direction,
    Velocity,
    Player,
    PlayerPart,
    Size,
    Sprite,
    Camera,
    Control,
) {
    (
        pos,
        Direction::Down,
        Velocity::new(0.0, 0.0),
        Player,
        PlayerPart::empty(),
        Size::new(GRID_SIZE, GRID_SIZE),
        Sprite::new(BLUE),
        Camera,
        Control,
    )
}

pub fn create_chaser(
    pos: Position,
) -> (
    Position,
    Direction,
    Velocity,
    Player,
    PlayerPart,
    Size,
    Sprite,
    Chase,
) {
    (
        pos,
        Direction::Down,
        Velocity::new(0.0, 0.0),
        Player,
        PlayerPart::empty(),
        Size::new(GRID_SIZE, GRID_SIZE),
        Sprite::new(YELLOW),
        Chase::new(),
    )
}

pub fn create_attack(
    pos: Position,
    dir: Direction,
) -> (Position, Direction, Velocity, Size, Sprite, Temporary) {
    const SPEED: f32 = 400.0;
    (
        pos,
        dir,
        match dir {
            Direction::Up => Velocity::new(0.0, -1.0 * SPEED),
            Direction::Down => Velocity::new(0.0, SPEED),
            Direction::Right => Velocity::new(SPEED, 0.0),
            Direction::Left => Velocity::new(-1.0 * SPEED, 0.0),
        },
        Size::new(GRID_SIZE, GRID_SIZE),
        Sprite::new(PURPLE),
        Temporary::die_after(0.5),
    )
}
