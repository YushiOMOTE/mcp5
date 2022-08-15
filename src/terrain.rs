use crate::chunk::Chunk;
use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use bevy_rapier3d::prelude::*;
use crossbeam_channel::{bounded, Receiver, Sender};
use std::collections::HashMap;

#[derive(Debug, Component)]
pub struct Loader {
    load_range: Vec3,
    update_range: Vec3,
    last_pos: Option<Vec3>,
    chunks: HashMap<Chunk, Entity>,
}

impl Loader {
    pub fn new() -> Self {
        Self {
            load_range: Chunk::size() * 4.0,
            update_range: Chunk::size() * 0.5,
            last_pos: None,
            chunks: HashMap::new(),
        }
    }

    pub fn try_update(&mut self, new_pos: Vec3) -> bool {
        match self.last_pos.as_mut() {
            Some(last_pos) => {
                let needs_update = {
                    let diff = (new_pos - *last_pos).abs();
                    diff.x > self.update_range.x
                        || diff.y > self.update_range.y
                        || diff.z > self.update_range.z
                };

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
        self.last_pos
            .map(|last_pos| Range::new(last_pos - self.load_range, last_pos + self.load_range))
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

struct ComputedChunk {
    entity: Entity,
    chunk: Chunk,
    mesh: Mesh,
}

impl ComputedChunk {
    fn new(entity: Entity, chunk: Chunk, mesh: Mesh) -> Self {
        Self {
            entity,
            chunk,
            mesh,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ComputedChunkReceiver(Receiver<ComputedChunk>);

#[derive(Debug, Clone)]
pub struct ComputedChunkSender(Sender<ComputedChunk>);

pub struct TerrainGenPlugin;

impl Plugin for TerrainGenPlugin {
    fn build(&self, app: &mut App) {
        let (tx, rx) = bounded(100);
        app.insert_resource(ComputedChunkSender(tx));
        app.insert_resource(ComputedChunkReceiver(rx));
    }
}

pub fn create_terrain(
    commands: &mut Commands,
    sender: ComputedChunkSender,
    chunk: Chunk,
) -> Entity {
    let task_pool = AsyncComputeTaskPool::get();

    let entity = commands.spawn().insert(chunk.clone()).id();

    task_pool
        .spawn(async move {
            let mesh = match chunk.generate_mesh() {
                Some(m) => m,
                None => return,
            };

            let computed_chunk = ComputedChunk::new(entity, chunk, mesh);
            let _ = sender.0.send(computed_chunk);
        })
        .detach();

    entity
}

pub fn render_terrain_system(
    loaders: Query<&Loader>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
    receiver: Res<ComputedChunkReceiver>,
) {
        // assume there's only one loader
    let loader = match loaders.iter().next() {
        Some(l) => l,
        None => return,
    };

    for chunk in receiver.0.try_iter() {
        if !loader.chunks.contains_key(&chunk.chunk) {
            // chunk is already removed
            continue;
        }

        commands
            .entity(chunk.entity)
            .insert(ColliderMassProperties::Density(100000.0))
            .insert(Collider::from_bevy_mesh(&chunk.mesh, &ComputedColliderShape::TriMesh).unwrap())
            .insert_bundle(PbrBundle {
                mesh: meshes.add(chunk.mesh),
                material: materials.add(Color::WHITE.into()),
                transform: Transform::from_translation(chunk.chunk.position()),
                ..default()
            });
    }
}

pub fn request_terrain_system(
    mut loaders: Query<(&Transform, &mut Loader)>,
    mut commands: Commands,
    sender: Res<ComputedChunkSender>,
) {
    // assume there's only one loader
    let (transform, mut loader) = match loaders.iter_mut().next() {
        Some((t, l)) => (*t, l),
        None => return,
    };

    let pos = transform.translation;

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

    let mut chunks: Vec<_> = (chunk_min.x..=chunk_max.x)
        .map(|x| {
            (chunk_min.y..=chunk_max.y)
                .map(move |y| (chunk_min.z..=chunk_max.z).map(move |z| (x, y, z)))
                .flatten()
        })
        .flatten()
        .map(|(x, y, z)| Chunk::new(x, y, z))
        .filter(|c| !c.is_empty())
        .collect();

    chunks.sort_by_key(|chunk| (chunk.position().distance(pos) * 1000.0) as i64);

    for chunk in chunks {
        if !range.contains(chunk.position()) {
            continue;
        }
        if loader.chunks.contains_key(&chunk) {
            continue;
        }

        let entity = create_terrain(&mut commands, (*sender).clone(), chunk.clone());

        loader.chunks.insert(chunk, entity);
    }
}
