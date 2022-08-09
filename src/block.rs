use crate::{
    components::{Position, Size},
    draw::Sprite,
};
use legion::{systems::CommandBuffer, *};
use macroquad::prelude::*;
use rapier3d::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Block;

pub fn create_block(
    rigid_body_set: &mut RigidBodySet,
    collider_set: &mut ColliderSet,
    pos: Position,
) -> (
    Position,
    Block,
    Size,
    Sprite,
    RigidBodyHandle,
    ColliderHandle,
) {
    let size = Size::new(8.0, 8.0, 8.0);

    let collider = ColliderBuilder::cuboid(size.x * 0.5, size.y * 0.5, size.z * 0.5)
        .mass(200.0)
        .friction(5.0)
        .build();

    let rigid_body = RigidBodyBuilder::dynamic()
        .translation(vector![pos.x, pos.y, pos.z])
        .gravity_scale(20.0)
        .build();
    let rigid_body_handle = rigid_body_set.insert(rigid_body);

    let collider_handle =
        collider_set.insert_with_parent(collider, rigid_body_handle, rigid_body_set);

    (
        pos,
        Block,
        Size::new(8.0, 8.0, 8.0),
        Sprite::plain(Color::new(0.4, 0.4, 0.4, 1.0)),
        rigid_body_handle,
        collider_handle,
    )
}

pub fn create_fixed_block(
    rigid_body_set: &mut RigidBodySet,
    collider_set: &mut ColliderSet,
    pos: Position,
) -> (
    Position,
    Block,
    Size,
    Sprite,
    RigidBodyHandle,
    ColliderHandle,
) {
    let size = Size::new(8.0, 8.0, 8.0);

    let collider = ColliderBuilder::cuboid(size.x * 0.5, size.y * 0.5, size.z * 0.5)
        .mass(5000.0)
        .friction(5.0)
        .build();

    let rigid_body = RigidBodyBuilder::dynamic()
        .translation(vector![pos.x, pos.y, pos.z])
        .gravity_scale(20.0)
        .dominance_group(5)
        .build();
    let rigid_body_handle = rigid_body_set.insert(rigid_body);

    let collider_handle =
        collider_set.insert_with_parent(collider, rigid_body_handle, rigid_body_set);

    (
        pos,
        Block,
        Size::new(8.0, 8.0, 8.0),
        Sprite::plain(Color::new(0.2, 0.2, 0.2, 1.0)),
        rigid_body_handle,
        collider_handle,
    )
}

#[system]
pub fn load_blocks(
    #[resource] rigid_body_set: &mut RigidBodySet,
    #[resource] collider_set: &mut ColliderSet,
    command_buffer: &mut CommandBuffer,
) {
    let mut seed = 5323u64;

    let mut rng = || {
        seed = (8253729 * seed) + 2396403;
        (seed % 700) as f32 + 100.0
    };

    for _ in 1..10 {
        command_buffer.push(create_block(
            rigid_body_set,
            collider_set,
            Position::new(rng(), rng(), 200.0),
        ));
    }

    for _ in 1..10 {
        command_buffer.push(create_fixed_block(
            rigid_body_set,
            collider_set,
            Position::new(rng(), rng(), 200.0),
        ));
    }
}
