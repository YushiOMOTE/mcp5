use legion::{
    systems::{Builder, CommandBuffer},
    *,
};
use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub struct Temporary {
    deadline: f64,
}

impl Temporary {
    pub fn die_after(after: f64) -> Self {
        Self::die_at(get_time() + after)
    }

    pub fn die_at(at: f64) -> Self {
        Self { deadline: at }
    }

    pub fn is_dead(&self) -> bool {
        get_time() > self.deadline
    }
}

#[system(for_each)]
fn clean_temporary(entity: &Entity, temp: &Temporary, command_buffer: &mut CommandBuffer) {
    if temp.is_dead() {
        command_buffer.remove(*entity);
    }
}

pub fn setup_systems(builder: &mut Builder) -> &mut Builder {
    builder.add_system(clean_temporary_system())
}
