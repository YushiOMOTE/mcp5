use legion::{systems::CommandBuffer, *};
use macroquad::prelude::*;
use rapier3d::prelude::*;

use crate::{
    components::{Position, Size},
    draw::Sprite,
    map::map_gen,
};

const SIZE: f32 = 8.0;

#[derive(Debug)]
pub struct Terrain;

fn color(level: f32) -> Color {
    if level <= 0.1 {
        Color::new(0.0, level * 2.0, 0.5 + level * 2.0, 1.0)
    } else if level > 0.1 && level <= 0.3 {
        Color::new(1.0 - level * 0.1, 1.0 - level, 1.0 - level, 1.0)
    } else if level > 0.3 && level <= 0.8 {
        Color::new(0.1, 1.0 - level, 0.1, 1.0)
    } else {
        Color::new(0.5 - (level - 0.8), 0.3 - (level - 0.8), 0.0, 1.0)
    }
}

pub fn create_terrain(
    rigid_body_set: &mut RigidBodySet,
    collider_set: &mut ColliderSet,
    pos: Position,
    level: f32,
) -> (
    Position,
    Size,
    Sprite,
    Terrain,
    RigidBodyHandle,
    ColliderHandle,
) {
    let half_heigh = (level * 10.0).floor() * SIZE + SIZE;
    let size = Size::new(SIZE, SIZE, half_heigh * 2.0);

    let collider = ColliderBuilder::cuboid(size.x * 0.5, size.y * 0.5, size.z * 0.5)
        .friction(0.0)
        .build();

    let rigid_body = RigidBodyBuilder::fixed()
        .translation(vector![pos.x, pos.y, pos.z])
        .build();
    let rigid_body_handle = rigid_body_set.insert(rigid_body);

    let collider_handle =
        collider_set.insert_with_parent(collider, rigid_body_handle, rigid_body_set);

    (
        pos,
        size,
        Sprite::new(color(level)),
        Terrain,
        rigid_body_handle,
        collider_handle,
    )
}

#[system]
pub fn load_terrain(
    #[resource] rigid_body_set: &mut RigidBodySet,
    #[resource] collider_set: &mut ColliderSet,
    command_buffer: &mut CommandBuffer,
) {
    let map = map_gen();

    map.map.iter().enumerate().for_each(|(i, level)| {
        let x = i as u64 % map.width;
        let y = i as u64 / map.width;

        let x = x as f32 * SIZE;
        let y = y as f32 * SIZE;

        command_buffer.push(create_terrain(
            rigid_body_set,
            collider_set,
            Position::new(x, y, 0.0),
            *level,
        ));
    });
}
