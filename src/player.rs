use crate::{
    ai::chase::Chase,
    camera::Camera,
    components::{Direction, Position, Size},
    control::Control,
    draw::Sprite,
    grid::GRID_SIZE,
    physics::{Body, Velocity},
    temporary::Temporary,
};
use legion::{systems::CommandBuffer, *};
use macroquad::prelude::*;
use rapier3d::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Player;

pub fn create_player(
    rigid_body_set: &mut RigidBodySet,
    collider_set: &mut ColliderSet,
    pos: Position,
) -> (
    Position,
    Direction,
    Player,
    Size,
    Sprite,
    Camera,
    Control,
    Body,
    RigidBodyHandle,
    ColliderHandle,
) {
    let size = Size::new(GRID_SIZE, GRID_SIZE, GRID_SIZE);

    let collider = ColliderBuilder::cuboid(size.x, size.y, size.z)
        .mass(100.0)
        .build();

    let rigid_body = RigidBodyBuilder::dynamic()
        .translation(vector![pos.x, pos.y, pos.z])
        .enabled_rotations(false, false, true)
        .can_sleep(false)
        .build();
    let rigid_body_handle = rigid_body_set.insert(rigid_body);

    let collider_handle =
        collider_set.insert_with_parent(collider, rigid_body_handle, rigid_body_set);

    (
        pos,
        Direction::Down,
        Player,
        size,
        Sprite::new(BLUE),
        Camera,
        Control,
        Body,
        rigid_body_handle,
        collider_handle,
    )
}

#[allow(unused)]
pub fn create_chaser(
    rigid_body_set: &mut RigidBodySet,
    collider_set: &mut ColliderSet,
    pos: Position,
) -> (Position, Direction, Player, Size, Sprite, Chase) {
    (
        pos,
        Direction::Down,
        Player,
        Size::new(GRID_SIZE, GRID_SIZE, GRID_SIZE),
        Sprite::new(YELLOW),
        Chase::new(),
    )
}

#[system]
pub fn load_player(
    #[resource] rigid_body_set: &mut RigidBodySet,
    #[resource] collider_set: &mut ColliderSet,
    command_buffer: &mut CommandBuffer,
) {
    command_buffer.push(create_player(
        rigid_body_set,
        collider_set,
        Position::new(120.0, 120.0, -500.0),
    ));
    // command_buffer.push(create_chaser(
    //     rigid_body_set,
    //     collider_set,
    //     Position::new(360.0, 360.0, 0.0),
    // ));
    // command_buffer.push(create_chaser(
    //     rigid_body_set,
    //     collider_set,
    //     Position::new(560.0, 560.0, 0.0),
    // ));
}
