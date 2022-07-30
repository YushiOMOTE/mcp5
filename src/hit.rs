use legion::{systems::CommandBuffer, world::SubWorld, *};
use macroquad::prelude::*;

use crate::components::Position;
use crate::player::Player;

#[derive(Debug)]
pub struct Hit {
    group_id: u64,
    value: i64,
    range: Rect,
}

impl Hit {
    pub fn new(group_id: u64, value: i64, range: Rect) -> Self {
        Self {
            group_id,
            value,
            range,
        }
    }
}

#[derive(Debug)]
pub struct Hitbox {
    group_id: u64,
    range: Rect,
}

impl Hitbox {
    pub fn new(group_id: u64, range: Rect) -> Self {
        Self { group_id, range }
    }
}

#[derive(Debug)]
pub struct Life {
    life: i64,
    max_life: i64,
}

impl Life {
    pub fn new(life: i64, max_life: i64) -> Self {
        Self { life, max_life }
    }

    pub fn update(&mut self, value: i64) -> bool {
        self.life = self.max_life.min((self.life + value).max(0));
        self.life == 0
    }
}

#[system]
#[read_component(Position)]
#[read_component(Hit)]
#[read_component(Player)]
#[read_component(Hitbox)]
#[write_component(Life)]
pub fn hit_check(
    world: &mut SubWorld,
    hits: &mut Query<(Entity, &Position, &Hit)>,
    players: &mut Query<(Entity, &Position, &Hitbox, &mut Life, &Player)>,
    command_buffer: &mut CommandBuffer,
) {
    let hits: Vec<_> = hits
        .iter(world)
        .map(|(e, pos, hit)| (*e, hit.range.offset(**pos), hit.value, hit.group_id))
        .collect();

    for (player_entity, pos, hitbox, life, _) in players.iter_mut(world) {
        let hitbox_range = hitbox.range.offset(**pos);
        for (hit_entity, _, value, _) in hits
            .iter()
            .filter(|(_, _, _, id)| *id != hitbox.group_id)
            .filter(|(_, range, _, _)| range.overlaps(&hitbox_range))
        {
            command_buffer.remove(*hit_entity);
            if life.update(*value) {
                command_buffer.remove(*player_entity);
            }
        }
    }
}
