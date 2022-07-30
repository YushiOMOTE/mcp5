use legion::*;
use macroquad::prelude::*;

use ai::chase::{chase_target_system, find_chase_target_system};
use block::{create_block, create_fixed_block};
use camera::update_camera_system;
use components::Position;
use control::control_system;
use grid::grid_system;
use hit::hit_check_system;
use interaction::{hold_block_system, player_block_collision_system, unhold_block_system};
use physics::update_positions_system;
use player::{create_chaser, create_player};
use sprite::draw_sprites_system;
use temporary::clean_temporary_system;

mod ai;
mod block;
mod camera;
mod components;
mod control;
mod grid;
mod hit;
mod interaction;
mod keymap;
mod physics;
mod player;
mod sprite;
mod temporary;

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

    world.push(create_player(Position::new(120.0, 120.0)));
    world.push(create_chaser(Position::new(360.0, 360.0)));

    world.extend(vec![
        create_block(Position::new(0.0, 0.0)),
        create_block(Position::new(0.0, 40.0)),
        create_block(Position::new(0.0, 80.0)),
        create_block(Position::new(0.0, 120.0)),
        create_block(Position::new(80.0, 80.0)),
        create_block(Position::new(160.0, 80.0)),
        create_block(Position::new(240.0, 80.0)),
    ]);

    world.extend(vec![
        create_fixed_block(Position::new(360.0, 0.0)),
        create_fixed_block(Position::new(360.0, 40.0)),
        create_fixed_block(Position::new(360.0, 80.0)),
        create_fixed_block(Position::new(360.0, 120.0)),
    ]);

    let mut schedule = Schedule::builder()
        .add_system(update_camera_system())
        .add_system(draw_sprites_system())
        .add_system(control_system())
        .add_system(player_block_collision_system())
        .add_system(update_positions_system())
        .add_system(hold_block_system())
        .add_system(unhold_block_system())
        .add_system(grid_system())
        .add_system(find_chase_target_system())
        .add_system(chase_target_system())
        .add_system(clean_temporary_system())
        .add_system(hit_check_system())
        .build();

    while !is_key_down(KeyCode::Escape) {
        clear_background(WHITE);

        schedule.execute(&mut world, &mut resources);

        next_frame().await
    }
}
