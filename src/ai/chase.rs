use legion::{systems::Builder, world::SubWorld, *};
use macroquad::prelude::*;
use std::collections::HashMap;

use crate::{components::Position, grid::GRID_SIZE, player::Player};

const THRESHOLD: f32 = GRID_SIZE * 20.0;
const SPEED: f32 = GRID_SIZE * 6.0;

#[derive(Debug)]
pub struct Chase {
    entity: Option<Entity>,
}

impl Chase {
    pub fn new() -> Self {
        Self { entity: None }
    }
}

#[system]
#[read_component(Player)]
#[read_component(Position)]
#[write_component(Chase)]
fn find_chase_target(
    world: &mut SubWorld,
    chasers: &mut Query<(Entity, &mut Chase, &Position, &Player)>,
    players: &mut Query<(Entity, &Position, &Player)>,
) {
    let players: Vec<_> = players.iter(world).map(|(e, p, _)| (*e, *p)).collect();

    for (entity, chase, pos, _) in chasers.iter_mut(world) {
        chase.entity = players
            .iter()
            .filter(|(e, _)| *e != *entity)
            .map(|(e, p)| (p.distance(**pos), *e))
            .filter(|(d, _)| *d <= THRESHOLD)
            .min_by_key(|(d, _)| *d as i32)
            .map(|(_, e)| e);
    }
}

#[system]
#[read_component(Player)]
#[write_component(Position)]
#[read_component(Chase)]
fn chase_target(
    world: &mut SubWorld,
    chasers: &mut Query<(&Chase, &mut Position, &Player)>,
    players: &mut Query<(Entity, &Position, &Player)>,
) {
    let players: HashMap<_, _> = players.iter(world).map(|(e, p, _)| (*e, *p)).collect();

    for (chase, pos, _) in chasers.iter_mut(world) {
        let target_entity = match chase.entity {
            Some(e) => e,
            None => continue,
        };
        let target_pos = match players.get(&target_entity) {
            Some(p) => p,
            _ => continue,
        };
        let norm = match (**target_pos - **pos).try_normalize() {
            Some(n) => n,
            None => continue,
        };

        let movement = norm * SPEED * get_frame_time();
        **pos += movement;
    }
}

pub fn setup_systems(builder: &mut Builder) -> &mut Builder {
    builder
        .add_system(find_chase_target_system())
        .add_system(chase_target_system())
}
