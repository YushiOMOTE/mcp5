use legion::*;
use macroquad::prelude::*;

use block::create_block;
use components::Position;
use grid::grid_system;
use interaction::{hold_block_system, player_block_collision_system, unhold_block_system};
use physics::update_positions_system;
use player::{control_player_system, create_player};
use sprite::draw_sprites_system;

mod block;
mod components;
mod grid;
mod interaction;
mod physics;
mod player;
mod sprite;

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

    world.extend(vec![
        create_block(Position::new(0.0, 0.0)),
        create_block(Position::new(0.0, 40.0)),
        create_block(Position::new(0.0, 80.0)),
        create_block(Position::new(0.0, 120.0)),
        create_block(Position::new(80.0, 80.0)),
        create_block(Position::new(160.0, 80.0)),
        create_block(Position::new(240.0, 80.0)),
    ]);

    let mut schedule = Schedule::builder()
        .add_system(draw_sprites_system())
        .add_system(control_player_system())
        .add_system(player_block_collision_system())
        .add_system(update_positions_system())
        .add_system(hold_block_system())
        .add_system(unhold_block_system())
        .add_system(grid_system())
        .build();

    while !is_key_down(KeyCode::Escape) {
        clear_background(WHITE);

        schedule.execute(&mut world, &mut resources);

        next_frame().await
    }
}
