use derive_deref::{Deref, DerefMut};
use legion::{
    systems::{Builder, CommandBuffer},
    *,
};
use macroquad::prelude::*;

use crate::components::Position;

#[derive(Clone, Copy, Debug, Deref, DerefMut)]
pub struct LastPosition(Vec3);

#[system(for_each)]
fn update_last_position(
    entity: &Entity,
    pos: &Position,
    last_pos: Option<&mut LastPosition>,
    command_buffer: &mut CommandBuffer,
) {
    match last_pos {
        Some(last_pos) => {
            last_pos.0 = **pos;
        }
        None => command_buffer.add_component(*entity, LastPosition(**pos)),
    }
}

pub fn setup_systems(builder: &mut Builder) -> &mut Builder {
    builder.add_system(update_last_position_system())
}
