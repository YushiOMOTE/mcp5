use legion::{systems::Builder, world::SubWorld, *};
use macroquad::prelude::*;
use noise::{NoiseFn, Perlin, Seedable};

use crate::{
    block::Block,
    components::{self, Position, Size},
    draw::Sprite,
    grid::GRID_SIZE,
};

pub struct Map {
    pub width: u64,
    pub height: u64,
    pub map: Vec<u64>,
}

pub struct Config {
    pub seed: u32,
    pub redistribution: f64,
    pub freq: f64,
    pub octaves: usize,
}

pub fn proc_gen(width: u64, height: u64, cfg: Config) -> Vec<u64> {
    let perlin = Perlin::new().set_seed(cfg.seed);
    let redist = cfg.redistribution;
    let freq = cfg.freq;
    let octaves = cfg.octaves;

    (0..width * height)
        .map(|i| {
            let x = i % width;
            let y = i / width;

            let nx = x as f64 / width as f64;
            let ny = y as f64 / width as f64;

            let value = (0..octaves).fold(0.0, |acc, n| {
                let power = 2.0f64.powf(n as f64);
                let modifier = 1.0 / power;
                acc + modifier * perlin.get([nx * freq * power, ny * freq * power])
            });
            let value = (value.powf(redist) + 1.0) / 2.0;

            ((value * 10.0) as u64).min(9)
        })
        .collect()
}

pub fn map_gen() -> Map {
    const MAP_WIDTH: u64 = 100;
    const MAP_HEIGHT: u64 = 100;

    let map = proc_gen(
        MAP_WIDTH,
        MAP_HEIGHT,
        Config {
            seed: 0,
            redistribution: 1.0,
            freq: 2.0,
            octaves: 3,
        },
    );

    Map {
        width: MAP_WIDTH,
        height: MAP_HEIGHT,
        map,
    }
}

#[derive(Debug)]
pub struct Terrain;

const TERRAIN_COLORS: [Color; 10] = [
    Color::new(0.0, 0.4, 0.8, 1.0),
    Color::new(0.2, 0.79, 1.0, 1.0),
    Color::new(1.0, 0.8, 0.6, 1.0),
    Color::new(1.0, 0.73, 0.4, 1.0),
    Color::new(0.4, 0.8, 0.0, 1.0),
    Color::new(0.29, 0.6, 0.0, 1.0),
    Color::new(0.8, 0.4, 0.0, 1.0),
    Color::new(0.6, 0.29, 0.0, 1.0),
    Color::new(0.4, 0.2, 0.0, 1.0),
    Color::new(0.2, 0.09, 0.0, 1.0),
];

pub fn create_terrain(position: Position, level: u64) -> (Position, Size, Sprite, Terrain) {
    (
        position,
        Size::new(GRID_SIZE, GRID_SIZE, GRID_SIZE),
        Sprite::new(TERRAIN_COLORS[level as usize]),
        Terrain,
    )
}

pub fn load_terrain(world: &mut World) {
    let map = map_gen();

    // Uncomment for debug map generation
    // map.map.iter().enumerate().for_each(|(i, v)| {
    //     if i % map.width as usize == 0 {
    //         println!()
    //     }
    //     print!("{}", v);
    // });

    map.map.iter().enumerate().for_each(|(i, level)| {
        let x = i as u64 % map.width;
        let y = i as u64 / map.width;

        let x = x as f32 * GRID_SIZE;
        let y = y as f32 * GRID_SIZE;
        let z = *level as f32 * GRID_SIZE * -1.0;

        let entity = world.push(create_terrain(Position::new(x, y, z), *level));

        if is_block_terrain(*level) {
            if let Some(mut e) = world.entry(entity) {
                e.add_component(Block);
            }
        }
    });
}

fn is_block_terrain(level: u64) -> bool {
    level < 2 || level > 5
}

#[system]
#[write_component(Position)]
#[read_component(Size)]
#[read_component(Terrain)]
pub fn terrain_collision(world: &mut SubWorld) {
    let mut things = <(&mut Position, &Size)>::query().filter(!component::<Terrain>());
    let mut terrains = <(&Position, &Size, &Terrain)>::query();

    let terrain_rects: Vec<_> = terrains
        .iter(world)
        .map(|(pos, size, _)| (*pos, components::to_rect(*pos, *size)))
        .collect();

    for (pos, size) in things.iter_mut(world) {
        let rect = components::to_rect(*pos, *size);
        let margined = Rect::new(rect.x + 4.0, rect.y + 4.0, rect.w - 8.0, rect.h - 8.0);
        let min_z = terrain_rects
            .iter()
            .filter(|(_, terrain_rect)| margined.overlaps(&terrain_rect))
            .map(|(pos, _)| (pos.z * 100.0) as i64)
            .min()
            .unwrap_or(0);
        pos.z = min_z as f32 / 100.0 - size.z;
    }
}

pub fn setup_systems(builder: &mut Builder) -> &mut Builder {
    builder.add_system(terrain_collision_system())
}
