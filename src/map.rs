use legion::World;
use macroquad::prelude::*;
use noise::{NoiseFn, Perlin, Seedable};

use crate::{
    block::Block,
    components::{Position, Size},
    grid::GRID_SIZE,
    sprite::Sprite,
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
        Size::new(GRID_SIZE, GRID_SIZE),
        Sprite::new(TERRAIN_COLORS[level as usize]),
        Terrain,
    )
}

pub fn create_hazard_terrain(
    position: Position,
    level: u64,
) -> (Position, Size, Sprite, Terrain, Block) {
    (
        position,
        Size::new(GRID_SIZE, GRID_SIZE),
        Sprite::new(TERRAIN_COLORS[level as usize]),
        Terrain,
        Block,
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
    let (terrain, hazard_terrain): (Vec<_>, Vec<_>) = map
        .map
        .iter()
        .enumerate()
        .map(|(i, level)| {
            let x = i as u64 % map.width;
            let y = i as u64 / map.width;

            let x = x as f32 * GRID_SIZE;
            let y = y as f32 * GRID_SIZE;

            (Position::new(x, y), *level)
        })
        .partition(|(_, l)| *l >= 2 && *l <= 5);

    world.extend(terrain.iter().map(|(pos, l)| create_terrain(*pos, *l)));
    world.extend(
        hazard_terrain
            .iter()
            .map(|(pos, l)| create_hazard_terrain(*pos, *l)),
    );
}
