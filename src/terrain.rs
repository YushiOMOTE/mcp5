use crate::chunk::Chunk;
use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use bevy_rapier3d::prelude::*;
use futures_lite::future;
use std::collections::HashMap;

fn mask_y(mut vec: Vec3) -> Vec3 {
    vec.y = 0.0;
    vec
}

#[derive(Debug, Component)]
pub struct Loader {
    load_radius: f32,
    update_radius: f32,
    last_pos: Option<Vec3>,
    chunks: HashMap<Chunk, Entity>,
}

impl Loader {
    pub fn new() -> Self {
        Self {
            load_radius: 96.0,
            update_radius: 16.0,
            last_pos: None,
            chunks: HashMap::new(),
        }
    }

    pub fn try_update(&mut self, new_pos: Vec3) -> bool {
        match self.last_pos.as_mut() {
            Some(last_pos) => {
                let needs_update = new_pos.distance(*last_pos) > self.update_radius;

                if needs_update {
                    *last_pos = new_pos;
                }

                needs_update
            }
            None => {
                self.last_pos = Some(new_pos);
                true
            }
        }
    }

    fn range(&self) -> Option<Range> {
        self.last_pos.map(|last_pos| {
            Range::new(
                mask_y(last_pos - self.load_radius),
                mask_y(last_pos + self.load_radius),
            )
        })
    }
}

struct Range {
    min: Vec3,
    max: Vec3,
}

impl Range {
    fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    fn contains(&self, pos: Vec3) -> bool {
        self.min.x <= pos.x
            && pos.x <= self.max.x
            && self.min.y <= pos.y
            && pos.y <= self.max.y
            && self.min.z <= pos.z
            && pos.z <= self.max.z
    }
}

#[derive(Component)]
pub struct ComputeMesh(Task<Mesh>);

pub fn create_terrain(commands: &mut Commands, chunk: Chunk) -> Entity {
    let task_pool = AsyncComputeTaskPool::get();

    commands
        .spawn()
        .insert(chunk.clone())
        .insert(ComputeMesh(task_pool.spawn(async move { chunk.generate_mesh() })))
        .id()
}

pub fn render_terrain_system(
    mut terrains: Query<(Entity, &Chunk, &mut ComputeMesh)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    for (entity, chunk, mut task) in &mut terrains {
        if let Some(mesh) = future::block_on(future::poll_once(&mut task.0)) {
            commands
                .entity(entity)
                .remove::<ComputeMesh>()
                .insert(Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh).unwrap())
                .insert_bundle(PbrBundle {
                    mesh: meshes.add(mesh.clone()),
                    material: materials.add(Color::GRAY.into()),
                    transform: Transform::from_translation(chunk.position()),
                    ..default()
                });
        }
    }
}

pub fn request_terrain_system(
    mut loaders: Query<(&Transform, &mut Loader)>,
    mut commands: Commands,
) {
    // assume there's only one loader
    let (transform, mut loader) = match loaders.iter_mut().next() {
        Some((t, l)) => (*t, l),
        None => return,
    };

    let pos = mask_y(transform.translation);

    if !loader.try_update(pos) {
        // no updates required
        return;
    }

    let range = match loader.range() {
        Some(l) => l,
        None => return,
    };

    loader.chunks.retain(|chunk, entity| {
        let in_range = range.contains(chunk.position());
        if !in_range {
            commands.entity(*entity).despawn();
        }
        in_range
    });

    let chunk_min = Chunk::from_world_coord(range.min);
    let chunk_max = Chunk::from_world_coord(range.max);

    for x in chunk_min.x..=chunk_max.x {
        for y in chunk_min.y..=chunk_max.y {
            for z in chunk_min.z..=chunk_max.z {
                let chunk = Chunk::new(x, y, z);

                if !range.contains(chunk.position()) {
                    continue;
                }
                if loader.chunks.contains_key(&chunk) {
                    continue;
                }

                let entity = create_terrain(&mut commands, chunk.clone());

                loader.chunks.insert(chunk, entity);
            }
        }
    }
}
