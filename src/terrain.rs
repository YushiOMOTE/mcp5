// use legion::{systems::CommandBuffer, world::SubWorld, *};
// use macroquad::prelude::*;
// use rapier3d::prelude::*;
use crate::map::{map_cfg, ProcGen};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::collections::{HashSet, VecDeque};

// use crate::{
//     components::{Position, Size},
//     draw::Sprite,
//     map::{map_cfg, ProcGen},
//     physics::RemoveBuffer,
//     textures::Textures,
// };

const WIDTH: f32 = 0.8;
const HEIGHT: f32 = 0.8;

#[derive(Debug, Component)]
pub struct Terrain;

#[derive(Debug, Component)]
pub struct Loader {
    last_pos: Vec3,
    range: Option<MapRange>,
}

impl Loader {
    pub fn new() -> Self {
        Self {
            last_pos: Vec3::new(0.0, 0.0, 0.0),
            range: None,
        }
    }
}

type Add = VecDeque<(i64, i64)>;
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

fn color(level: f32) -> Color {
    if level <= 0.1 {
        Color::rgba(0.0, level * 2.0, 0.5 + level * 2.0, 1.0)
    } else if level > 0.1 && level <= 0.3 {
        Color::rgba(1.0 - level * 0.1, 1.0 - level, 1.0 - level, 1.0)
    } else if level > 0.3 && level <= 0.8 {
        Color::rgba(0.1, 1.0 - level, 0.1, 1.0)
    } else {
        Color::rgba(0.5 - (level - 0.8), 0.3 - (level - 0.8), 0.0, 1.0)
    }
}

pub fn create_terrain(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    transform: Transform,
    level: f32,
) {
    let half_heigh = (level * 10.0).floor() * HEIGHT + HEIGHT;

    commands
        .spawn()
        .insert(Terrain)
        .insert(Collider::cuboid(WIDTH * 0.5, half_heigh, WIDTH * 0.5))
        .insert_bundle(PbrBundle {
            mesh: meshes.add(shape::Box::new(WIDTH, half_heigh * 2.0, WIDTH).into()),
            material: materials.add(color(level).into()),
            transform,
            ..default()
        });
}

#[derive(Default)]
pub struct Pending {
    pub add: VecDeque<(i64, i64)>,
    pub remove: HashSet<(i64, i64)>,
}

pub fn request_terrain_system(
    mut pending: ResMut<Pending>,
    mut loaders: Query<(&Transform, &mut Loader)>,
) {
    let (transform, mut loader) = match loaders.iter_mut().next() {
        Some((t, l)) => (*t, l),
        None => return,
    };

    if transform.translation.distance(loader.last_pos) <= 3.0 {
        return;
    }

    loader.last_pos = transform.translation;

    let w = 100.0 * 0.5;
    let h = 100.0 * 0.5;
    let x = transform.translation.x - w * 0.5 - 2.0;
    let z = transform.translation.z - h * 0.5 - 2.0;

    let tx = (x / WIDTH) as i64;
    let tz = (z / WIDTH) as i64;
    let tw = (w / WIDTH) as i64;
    let th = (h / WIDTH) as i64;

    let range = MapRange::new(tx, tz, tw, th);

    let (add, remove) = match &loader.range {
        Some(old) => range.diff(&old),
        None => range.to_add(),
    };

    println!(
        "+{}({}),-{}({})",
        add.len(),
        pending.add.len(),
        remove.len(),
        pending.remove.len()
    );

    pending.add.retain(|v| !remove.contains(v));
    pending.remove.retain(|v| !add.contains(v));
    pending.add.extend(add);
    pending.remove.extend(remove);

    loader.range = Some(range);
}

pub fn load_terrain_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    terrain: Query<(Entity, &Transform, &Terrain)>,
    mut pending: ResMut<Pending>,
) {
    let mut count = 0;

    for (entity, terrain_transform, _) in &terrain {
        let x = (terrain_transform.translation.x / WIDTH) as i64;
        let z = (terrain_transform.translation.z / WIDTH) as i64;

        if pending.remove.remove(&(x, z)) {
            commands.entity(entity).despawn();
            count += 1;
        }

        if count >= 50 {
            return;
        }
    }

    let gen = ProcGen::new(map_cfg());
    let mut count = 0;
    while let Some((x, z)) = pending.add.pop_front() {
        let level = gen.gen(x, z);

        create_terrain(
            &mut commands,
            &mut meshes,
            &mut materials,
            Transform::from_xyz(x as f32 * WIDTH, 0.0, z as f32 * WIDTH),
            level,
        );

        count += 1;

        if count >= 50 {
            return;
        }
    }
}
