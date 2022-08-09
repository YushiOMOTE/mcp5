use crate::{
    ai::chase::Chase,
    camera::Camera,
    components::{Position, Size},
    control::Control,
    draw::Sprite,
    terrain::Loader,
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
    Player,
    Size,
    Sprite,
    Camera,
    Control,
    Loader,
    RigidBodyHandle,
    ColliderHandle,
) {
    let size = Size::new(8.0, 8.0, 16.0);

    let collider = ColliderBuilder::cuboid(size.x * 0.5, size.y * 0.5, size.z * 0.5)
        .mass(100.0)
        .friction(0.0)
        .friction_combine_rule(CoefficientCombineRule::Min)
        .build();

    let rigid_body = RigidBodyBuilder::dynamic()
        .translation(vector![pos.x, pos.y, pos.z])
        .enabled_rotations(false, false, true)
        .can_sleep(false)
        .gravity_scale(20.0)
        .build();
    let rigid_body_handle = rigid_body_set.insert(rigid_body);

    let collider_handle =
        collider_set.insert_with_parent(collider, rigid_body_handle, rigid_body_set);

    (
        pos,
        Player,
        size,
        Sprite::plain(RED),
        Camera,
        Control,
        Loader::new(),
        rigid_body_handle,
        collider_handle,
    )
}

#[allow(unused)]
pub fn create_chaser(
    rigid_body_set: &mut RigidBodySet,
    collider_set: &mut ColliderSet,
    pos: Position,
) -> (Position, Player, Size, Sprite, Chase) {
    (
        pos,
        Player,
        Size::new(8.0, 8.0, 8.0),
        Sprite::plain(YELLOW),
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
        Position::new(120.0, 120.0, 200.0),
    ));
}
