use noise::{NoiseFn, Perlin, Seedable};

pub fn local_level_cfg() -> GenConfig {
    GenConfig {
        seed: 0,
        redistribution: 1.0,
        freq: 20_000.0,
        octaves: 3,
        max_width: 1_000_000,
        origin: (500_000, 500_000),
    }
}

pub fn global_level_cfg() -> GenConfig {
    GenConfig {
        seed: 0,
        redistribution: 1.0,
        freq: 2_000.0,
        octaves: 4,
        max_width: 1_000_000,
        origin: (500_000, 500_000),
    }
}

pub struct GenConfig {
    pub seed: u32,
    pub redistribution: f64,
    pub freq: f64,
    pub octaves: usize,
    pub max_width: i64,
    pub origin: (i64, i64),
}

pub struct ProcGen {
    perlin: Perlin,
    cfg: GenConfig,
}

impl ProcGen {
    pub fn new(cfg: GenConfig) -> Self {
        Self {
            perlin: Perlin::new().set_seed(cfg.seed),
            cfg: cfg,
        }
    }

    pub fn gen(&self, x: i64, y: i64) -> f32 {
        let x = x + self.cfg.origin.0;
        let y = y + self.cfg.origin.1;

        let redist = self.cfg.redistribution;
        let freq = self.cfg.freq;
        let octaves = self.cfg.octaves;

        let nx = x as f64 / self.cfg.max_width as f64;
        let ny = y as f64 / self.cfg.max_width as f64;

        let value = (0..octaves).fold(0.0, |acc, n| {
            let power = 2.0f64.powf(n as f64);
            let modifier = 1.0 / power;
            acc + modifier * self.perlin.get([nx * freq * power, ny * freq * power])
        });

        (((value.powf(redist) + 1.0) / 2.0) as f32)
            .max(0.0)
            .min(1.0)
    }
}
