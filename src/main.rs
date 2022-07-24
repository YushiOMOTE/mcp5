use legion::world::SubWorld;
use legion::{systems::CommandBuffer, *};
use macroquad::prelude::*;

use components::{merge_rects, Direction, Position, Size, Sprite, Velocity};

mod components;

#[derive(Clone, Copy, Debug)]
pub struct Block;

#[derive(Clone, Copy, Debug)]
pub struct Player;

#[derive(Clone, Copy, Debug)]
pub struct PlayerPart;

fn create_player(
    pos: Position,
) -> (
    Position,
    Direction,
    Velocity,
    Player,
    PlayerPart,
    Size,
    Sprite,
) {
    (
        pos,
        Direction::Down,
        Velocity::new(0.0, 0.0),
        Player,
        PlayerPart,
        Size::new(40.0, 40.0),
        Sprite::new(BLUE),
    )
}

fn create_block(pos: Position) -> (Position, Velocity, Block, Size, Sprite) {
    (
        pos,
        Velocity::new(0.0, 0.0),
        Block,
        Size::new(40.0, 40.0),
        Sprite::new(RED),
    )
}

#[system(for_each)]
fn update_positions(pos: &mut Position, vel: &Velocity) {
    pos.x += vel.x * get_frame_time();
    pos.y += vel.y * get_frame_time();
}

#[system(for_each)]
fn control_player(pos: &mut Position, dir: Option<&mut Direction>, _: &PlayerPart) {
    let step = 1.0;

    if let Some(dir) = dir {
        if is_key_down(KeyCode::Down) {
            dir.down();
        }
        if is_key_down(KeyCode::Up) {
            dir.up();
        }
        if is_key_down(KeyCode::Left) {
            dir.left();
        }
        if is_key_down(KeyCode::Right) {
            dir.right();
        }
    }

    if is_key_down(KeyCode::Down) {
        pos.y += step;
    }
    if is_key_down(KeyCode::Up) {
        pos.y -= step;
    }
    if is_key_down(KeyCode::Left) {
        pos.x -= step;
    }
    if is_key_down(KeyCode::Right) {
        pos.x += step;
    }
}

#[system]
fn hold_block(
    world: &mut SubWorld,
    players: &mut Query<(&Position, &Size, &PlayerPart)>,
    blocks: &mut Query<(Entity, &Position, &Size, &Block)>,
    command_buffer: &mut CommandBuffer,
) {
    let player_rects = players
        .iter(world)
        .map(|(pos, size, _)| components::to_rect(*pos, *size));

    if is_key_pressed(KeyCode::Z) {
        for player_rect in player_rects {
            for (entity, pos, size, _) in blocks.iter(world) {
                let block_rect = components::to_rect(*pos, *size);
                if player_rect.overlaps(&block_rect) {
                    command_buffer.add_component(*entity, PlayerPart);
                    command_buffer.remove_component::<Block>(*entity);
                }
            }
        }
    }
}

#[system]
#[read_component(Entity)]
#[read_component(PlayerPart)]
fn unhold_block(world: &mut SubWorld, command_buffer: &mut CommandBuffer) {
    let mut query = <(Entity, &PlayerPart)>::query().filter(!component::<Player>());

    if is_key_released(KeyCode::Z) {
        for (entity, _) in query.iter(world) {
            command_buffer.add_component(*entity, Block);
            command_buffer.remove_component::<PlayerPart>(*entity);
        }
    }
}

#[system]
fn player_block_collision(
    world: &mut SubWorld,
    players_pos: &mut Query<(&mut Position, &PlayerPart)>,
    players: &mut Query<(&Position, &Size, &PlayerPart)>,
    blocks: &mut Query<(&Position, &Size, &Block)>,
) {
    let block_rects = blocks
        .iter(world)
        .map(|(pos, size, _)| (components::to_rect(*pos, *size)));
    let player_rects = players
        .iter(world)
        .map(|(pos, size, _)| components::to_rect(*pos, *size));

    let player_rect = match merge_rects(player_rects) {
        Some(r) => r,
        None => return,
    };
    let adjusted_player_rect = adjust_player_rect(player_rect, block_rects);
    let adjustment = adjusted_player_rect.point() - player_rect.point();

    // Update player position
    for (pos, _) in players_pos.iter_mut(world) {
        **pos += adjustment;
    }
}

/// Check collision and return adjusted rect
fn adjust_player_rect<T: std::iter::Iterator<Item = Rect>>(
    player_rect: Rect,
    block_rects: T,
) -> Rect {
    block_rects.fold(player_rect, |player_rect, block_rect| {
        let margin = Vec2::new(player_rect.w * 0.45, player_rect.h * 0.45);
        let collision_info = match check_collision(&player_rect, &block_rect, margin) {
            Some(c) => c,
            None => return player_rect,
        };

        player_rect.offset(collision_info.adjustment)
    })
}

struct CollisionInfo {
    adjustment: Vec2,
}

fn check_collision(player_rect: &Rect, block_rect: &Rect, margin: Vec2) -> Option<CollisionInfo> {
    // collision detection with smaller rectangle otherwise player gets stuck for excessive collision due to rounding errors
    if !components::to_inner_rect(*player_rect).overlaps(&components::to_inner_rect(*block_rect)) {
        return None;
    }

    let overlap = match player_rect.intersect(*block_rect) {
        Some(rect) => rect,
        None => return None,
    };

    let player_center = components::center(*player_rect);
    let touch_up = overlap.y <= player_center.y;
    let touch_right = overlap.x <= player_center.x;

    let (adjust_x, adjust_y) = if overlap.w < overlap.h {
        let adjust_x = if touch_right {
            overlap.w
        } else {
            -1.0 * overlap.w
        };

        assert!(margin.y < player_rect.h / 2.0);
        if overlap.h <= margin.y {
            let adjust_y = if touch_up {
                overlap.h
            } else {
                -1.0 * overlap.h
            };
            (adjust_x, adjust_y)
        } else if overlap.h >= player_rect.h - margin.y {
            let adjust_y = if player_rect.y <= block_rect.y {
                player_rect.h - overlap.h
            } else {
                overlap.h - player_rect.h
            };
            (adjust_x, adjust_y)
        } else {
            (adjust_x, 0.0)
        }
    } else {
        let adjust_y = if touch_up {
            overlap.h
        } else {
            -1.0 * overlap.h
        };

        assert!(margin.x < player_rect.w / 2.0);
        if overlap.w <= margin.x {
            let adjust_x = if touch_right {
                overlap.w
            } else {
                -1.0 * overlap.w
            };
            (adjust_x, adjust_y)
        } else if overlap.w >= player_rect.w - margin.x {
            let adjust_x = if player_rect.x <= block_rect.x {
                player_rect.w - overlap.w
            } else {
                overlap.w - player_rect.w
            };
            (adjust_x, adjust_y)
        } else {
            (0.0, adjust_y)
        }
    };

    Some(CollisionInfo {
        adjustment: Vec2::new(adjust_x, adjust_y),
    })
}

#[system(for_each)]
fn draw_sprites(pos: &Position, size: &Size, sprite: &Sprite) {
    draw_rectangle(pos.x, pos.y, size.x, size.y, sprite.color());
}

#[macroquad::main("gf")]
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
        .build();

    while !is_key_down(KeyCode::Escape) {
        clear_background(WHITE);

        schedule.execute(&mut world, &mut resources);

        next_frame().await
    }
}
