use legion::{systems::Builder, *};
use macroquad::prelude::*;

#[allow(unused)]
mod ai;
mod block;
mod camera;
mod components;
mod control;
mod draw;
mod grid;
mod keymap;
mod physics;
mod player;
#[allow(unused)]
mod temporary;
mod terrain;

fn window_conf() -> Conf {
    Conf {
        window_title: "mcp5".into(),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut world = World::default();
    let mut resources = Resources::default();

    physics::setup_resources(&mut resources);

    let mut init = Schedule::builder()
        .add_system(terrain::load_terrain_system())
        .add_system(player::load_player_system())
        .build();

    let mut schedule = Schedule::builder()
        .add_system(physics::update_physics_system())
        .add_system(control::control_system())
        .add_system(camera::update_camera_system())
        .add_system(draw::draw_system())
        .build();

    init.execute(&mut world, &mut resources);

    while !is_key_down(KeyCode::Escape) {
        clear_background(WHITE);

        schedule.execute(&mut world, &mut resources);

        next_frame().await
    }
}
