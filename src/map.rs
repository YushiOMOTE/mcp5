use noise::{NoiseFn, Perlin, Seedable};

pub struct Map {
    pub width: u64,
    pub height: u64,
    pub map: Vec<f32>,
}

pub struct Config {
    pub seed: u32,
    pub redistribution: f64,
    pub freq: f64,
    pub octaves: usize,
}

const MAX_WIDTH: u64 = 1_000_000;

pub fn map_cfg() -> Config {
    Config {
        seed: 0,
        redistribution: 1.0,
        freq: 20_000.0,
        octaves: 3,
    }
}

pub fn gen(base_x: u64, base_y: u64, width: u64, height: u64, cfg: Config) -> Vec<f32> {
    let perlin = Perlin::new().set_seed(cfg.seed);
    let redist = cfg.redistribution;
    let freq = cfg.freq;
    let octaves = cfg.octaves;

    (0..width)
        .map(|x| (0..height).map(move |y| (x, y)))
        .flatten()
        .map(|(x, y)| {
            let x = base_x + x;
            let y = base_y + y;

            let nx = x as f64 / MAX_WIDTH as f64;
            let ny = y as f64 / MAX_WIDTH as f64;

            let value = (0..octaves).fold(0.0, |acc, n| {
                let power = 2.0f64.powf(n as f64);
                let modifier = 1.0 / power;
                acc + modifier * perlin.get([nx * freq * power, ny * freq * power])
            });
            (((value.powf(redist) + 1.0) / 2.0) as f32).max(0.0)
        })
        .collect()
}

pub fn map_gen() -> Map {
    const MAP_WIDTH: u64 = 100;
    const MAP_HEIGHT: u64 = 100;

    let map = gen(0, 0, MAP_WIDTH, MAP_HEIGHT, map_cfg());

    Map {
        width: MAP_WIDTH,
        height: MAP_HEIGHT,
        map,
    }
}
