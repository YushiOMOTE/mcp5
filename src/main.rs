use legion::{systems::Builder, *};
use macroquad::prelude::*;

use block::{create_block, create_fixed_block};
use components::Position;
use player::{create_chaser, create_player};
use terrain::load_terrain;

mod ai;
mod block;
mod camera;
mod components;
mod control;
mod draw;
mod grid;
mod hit;
mod interaction;
mod keymap;
mod motion;
mod physics;
mod player;
mod temporary;
mod terrain;

fn window_conf() -> Conf {
    Conf {
        window_title: "mcp5".into(),
        ..Default::default()
    }
}

fn setup_systems(builder: &mut Builder) -> &mut Builder {
    camera::setup_systems(builder);
    draw::setup_systems(builder);
    control::setup_systems(builder);
    physics::setup_systems(builder);
    interaction::setup_systems(builder);
    grid::setup_systems(builder);
    ai::setup_systems(builder);
    temporary::setup_systems(builder);
    hit::setup_systems(builder)
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut world = World::default();
    let mut resources = Resources::default();

    world.push(create_player(Position::new(1120.0, 1120.0, 0.0)));
    world.push(create_chaser(Position::new(1360.0, 1360.0, 0.0)));
    world.push(create_chaser(Position::new(360.0, 360.0, 0.0)));

    world.extend(vec![
        create_block(Position::new(1000.0, 1000.0, 0.0)),
        create_block(Position::new(1000.0, 1040.0, 0.0)),
        create_block(Position::new(1000.0, 1080.0, 0.0)),
        create_block(Position::new(1000.0, 1120.0, 0.0)),
        create_block(Position::new(1080.0, 1080.0, 0.0)),
        create_block(Position::new(1160.0, 1080.0, 0.0)),
        create_block(Position::new(1240.0, 1080.0, 0.0)),
    ]);

    world.extend(vec![
        create_fixed_block(Position::new(1360.0, 1000.0, 0.0)),
        create_fixed_block(Position::new(1360.0, 1040.0, 0.0)),
        create_fixed_block(Position::new(1360.0, 1080.0, 0.0)),
        create_fixed_block(Position::new(1360.0, 1120.0, 0.0)),
    ]);

    load_terrain(&mut world);

    let mut builder = Schedule::builder();
    let mut schedule = setup_systems(&mut builder).build();

    while !is_key_down(KeyCode::Escape) {
        clear_background(WHITE);

        schedule.execute(&mut world, &mut resources);

        next_frame().await
    }
}
