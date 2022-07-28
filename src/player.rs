use crate::{
    ai::chase::Chase,
    camera::Camera,
    components::{Direction, Position, Size},
    control::Control,
    grid::GRID_SIZE,
    physics::Velocity,
    sprite::Sprite,
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
