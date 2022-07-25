use legion::world::SubWorld;
use legion::{systems::CommandBuffer, *};
use macroquad::prelude::*;

use crate::block::Block;
use crate::components::{self, merge_rects, to_rect, Direction, Position, Size};
use crate::grid::Grid;
use crate::player::{Player, PlayerPart};

#[system]
pub fn hold_block(
    world: &mut SubWorld,
    players: &mut Query<(&Position, &Size, &Direction, &PlayerPart)>,
    blocks: &mut Query<(Entity, &Position, &Size, &Block)>,
    command_buffer: &mut CommandBuffer,
) {
    let player_rects = players
        .iter(world)
        .map(|(pos, size, dir, _)| (to_rect(*pos, *size), dir));

    if is_key_pressed(KeyCode::Z) {
        for (player_rect, dir) in player_rects {
            for (entity, pos, size, _) in blocks.iter(world) {
                let block_rect = components::to_rect(*pos, *size);
                if let Some(intersect) = player_rect.intersect(block_rect) {
                    let same_x = player_rect.x == intersect.x;
                    let same_y = player_rect.y == intersect.y;
                    let same_w = player_rect.w == intersect.w;
                    let same_h = player_rect.h == intersect.h;

                    match (same_x, same_y, same_w, same_h, dir) {
                        (_, true, true, _, Direction::Up)
                        | (_, false, true, _, Direction::Down)
                        | (true, _, _, true, Direction::Left)
                        | (false, _, _, true, Direction::Right) => {
                            command_buffer.add_component(*entity, PlayerPart);
                            command_buffer.remove_component::<Block>(*entity);
                            command_buffer.remove_component::<Grid>(*entity);
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

#[system]
#[read_component(Entity)]
#[read_component(PlayerPart)]
pub fn unhold_block(world: &mut SubWorld, command_buffer: &mut CommandBuffer) {
    let mut query = <(Entity, &PlayerPart)>::query().filter(!component::<Player>());

    if is_key_released(KeyCode::Z) {
        for (entity, _) in query.iter(world) {
            command_buffer.add_component(*entity, Block);
            command_buffer.add_component(*entity, Grid);
            command_buffer.remove_component::<PlayerPart>(*entity);
        }
    }
}

#[system]
pub fn player_block_collision(
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
