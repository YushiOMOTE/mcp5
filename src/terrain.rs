use legion::{systems::CommandBuffer, world::SubWorld, *};
use macroquad::prelude::*;
use rapier3d::prelude::*;
use std::collections::HashSet;

use crate::{
    components::{Position, Size},
    draw::Sprite,
    map::{map_cfg, ProcGen},
    physics::RemoveBuffer,
};

const SIZE: f32 = 8.0;

#[derive(Debug)]
pub struct Terrain;

#[derive(Debug)]
pub struct Loader {
    last_time: f64,
    last_pos: Vec3,
    range: Option<MapRange>,
}

impl Loader {
    pub fn new() -> Self {
        Self {
            last_time: get_time(),
            last_pos: vec3(0.0, 0.0, 0.0),
            range: None,
        }
    }
}

type Add = Vec<(i64, i64)>;
type Remove = HashSet<(i64, i64)>;

#[derive(Debug)]
pub struct MapRange {
    pub base_x: i64,
    pub base_y: i64,
    pub width: i64,
    pub height: i64,
}

impl MapRange {
    fn new(base_x: i64, base_y: i64, width: i64, height: i64) -> Self {
        Self {
            base_x,
            base_y,
            width,
            height,
        }
    }

    fn contains(&self, pos: (i64, i64)) -> bool {
        pos.0 >= self.base_x
            && pos.0 < self.base_x + self.width
            && pos.1 >= self.base_y
            && pos.1 < self.base_y + self.height
    }

    fn diff(&self, old: &MapRange) -> (Add, Remove) {
        let add = (0..self.width)
            .map(|x| (0..self.height).map(move |y| (x + self.base_x, y + self.base_y)))
            .flatten()
            .filter(|p| !old.contains(*p))
            .collect();
        let remove = (0..old.width)
            .map(|x| (0..old.height).map(move |y| (x + old.base_x, y + old.base_y)))
            .flatten()
            .filter(|p| !self.contains(*p))
            .collect();

        (add, remove)
    }

    fn to_add(&self) -> (Add, Remove) {
        (
            (0..self.width)
                .map(|x| (0..self.height).map(move |y| (x + self.base_x, y + self.base_y)))
                .flatten()
                .collect(),
            HashSet::new(),
        )
    }
}

pub fn create_texture(color: Color) -> Texture2D {
    let bytes: Vec<u8> = (0..16)
        .map(|x| (0..16).map(move |y| (x, y)))
        .flatten()
        .map(|(x, y)| {
            let rgba: [u8; 4] = color.into();
            let rgba = if x == 0 || y == 0 || x == 15 || y == 15 {
                [rgba[0] / 5 * 4, rgba[1] / 5 * 4, rgba[2] / 5 * 4, rgba[3]]
            } else {
                rgba
            };
            rgba.into_iter()
        })
        .flatten()
        .collect();

    let texture = Texture2D::from_rgba8(16, 16, &bytes);

    texture
}

fn sprite(level: f32) -> Sprite {
    let color = if level <= 0.1 {
        Color::new(0.0, level * 2.0, 0.5 + level * 2.0, 1.0)
    } else if level > 0.1 && level <= 0.3 {
        Color::new(1.0 - level * 0.1, 1.0 - level, 1.0 - level, 1.0)
    } else if level > 0.3 && level <= 0.8 {
        Color::new(0.1, 1.0 - level, 0.1, 1.0)
    } else {
        Color::new(0.5 - (level - 0.8), 0.3 - (level - 0.8), 0.0, 1.0)
    };

    let texture = create_texture(color);

    Sprite::with_texture(WHITE, texture)
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
        sprite(level),
        Terrain,
        rigid_body_handle,
        collider_handle,
    )
}

#[system]
pub fn load_terrain(
    world: &mut SubWorld,
    loaders: &mut Query<(&Position, &mut Loader)>,
    terrain: &mut Query<(Entity, &Position, &RigidBodyHandle, &Terrain)>,
    #[resource] rigid_body_set: &mut RigidBodySet,
    #[resource] collider_set: &mut ColliderSet,
    #[resource] remove_buffer: &mut RemoveBuffer,
    command_buffer: &mut CommandBuffer,
) {
    let (pos, loader) = match loaders.iter_mut(world).next() {
        Some((pos, loader)) => (*pos, loader),
        None => return,
    };

    let now = get_time();
    if now - loader.last_time <= 0.03 {
        return;
    }
    if pos.distance(loader.last_pos) <= 3.0 {
        return;
    }

    loader.last_time = now;
    loader.last_pos = *pos;

    let w = screen_width() * 0.5;
    let h = screen_height() * 0.5;
    let x = pos.x - w * 0.5;
    let y = pos.y - h * 0.5;

    let tx = (x / SIZE) as i64;
    let ty = (y / SIZE) as i64;
    let tw = (w / SIZE) as i64;
    let th = (h / SIZE) as i64;

    let range = MapRange::new(tx, ty, tw, th);

    let (add, remove) = match &loader.range {
        Some(old) => range.diff(&old),
        None => range.to_add(),
    };

    loader.range = Some(range);

    for (entity, terrain_pos, rigid_body_handle, _) in terrain.iter(world) {
        let x = (terrain_pos.x / SIZE) as i64;
        let y = (terrain_pos.y / SIZE) as i64;

        if remove.contains(&(x, y)) {
            command_buffer.remove(*entity);
            remove_buffer.push(*rigid_body_handle);
        }
    }

    let gen = ProcGen::new(map_cfg());

    for (x, y) in add {
        let level = gen.gen(x, y);

        command_buffer.push(create_terrain(
            rigid_body_set,
            collider_set,
            Position::new(x as f32 * SIZE, y as f32 * SIZE, 0.0),
            level,
        ));
    }
}
