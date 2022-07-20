use legion::*;
use legion::world::SubWorld;
use macroquad::prelude::*;

use components::{Direction, Position, Size, Sprite, Velocity};

mod components;

#[derive(Clone, Copy, Debug)]
pub struct Block;

#[derive(Clone, Copy, Debug)]
pub struct HeldEntity {
    entity: Entity,
    relative_pos: Vec2,
}

impl HeldEntity {
    fn new(entity: Entity, relative_pos: Vec2) -> Self {
        Self {
            entity,
            relative_pos,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Player {
    holding: bool,
    holding_entity: Option<HeldEntity>,
}

impl Player {
    fn new() -> Self {
        Self {
            holding: false,
            holding_entity: None,
        }
    }

    fn hold_block(&mut self) {
        self.holding = true;
    }

    fn is_holding(&self, entity: Entity) -> bool {
        match self.holding_entity {
            Some(e) if e.entity == entity => true,
            _ => false,
        }
    }

    fn held_entity(&self) -> Option<HeldEntity> {
        self.holding_entity
    }

    fn face(&mut self, new_entity: Entity, relative_pos: Vec2) {
        if !self.holding {
            return;
        }

        if self.holding_entity.is_none() {
            self.holding_entity = Some(HeldEntity::new(new_entity, relative_pos));
            println!("Take {:?}", self.holding_entity);
        }
    }

    fn release_block(&mut self) {
        println!("Release {:?}", self.holding_entity);
        self.holding = false;
        self.holding_entity = None;
    }

    fn step(&self) -> f32 {
        5.0
    }
}

fn create_player(pos: Position) -> (Position, Direction, Velocity, Player, Size, Sprite) {
    (pos, Direction::Down, Velocity::new(0.0, 0.0), Player::new(), Size::new(40.0, 40.0), Sprite::new(BLUE))
}

fn create_block(pos: Position) -> (Position, Velocity, Block, Size, Sprite) {
    (pos, Velocity::new(0.0, 0.0), Block, Size::new(40.0, 40.0), Sprite::new(RED))
}

#[system(for_each)]
fn update_positions(pos: &mut Position, vel: &Velocity) {
    pos.x += vel.x * get_frame_time();
    pos.y += vel.y * get_frame_time();
}

#[system(for_each)]
fn control_player(pos: &mut Position, dir: &mut Direction, player: &mut Player) {
    let step = player.step();

    if is_key_down(KeyCode::Down) {
        pos.y += step;
        dir.down();
    }
    if is_key_down(KeyCode::Up) {
        pos.y -= step;
        dir.up();
    }
    if is_key_down(KeyCode::Left) {
        pos.x -= step;
        dir.left();
    }
    if is_key_down(KeyCode::Right) {
        pos.x += step;
        dir.right();
    }

    if is_key_pressed(KeyCode::Z) {
        player.hold_block();
    }
    if is_key_released(KeyCode::Z) {
        player.release_block();
    }
}

#[system]
fn held_block_update(world: &mut SubWorld, players: &mut Query<(&Position, &Player)>, blocks: &mut Query<(Entity, &mut Position, &Block)>) {
    let players: Vec<_> = players.iter(world).map(|(pos, player)| (*pos, player.clone())).collect();

    for (player_pos, player) in players {
        for (entity, block_pos, _) in blocks.iter_mut(world) {
            let held_entity = match player.held_entity() {
                Some(e) if e.entity == *entity => e,
                _ => continue
            };

            **block_pos = *player_pos + held_entity.relative_pos;
        }
    }
}

#[system]
fn player_block_collision(world: &mut SubWorld, players: &mut Query<(&mut Position, &Size, &mut Player)>, blocks: &mut Query<(Entity, &Position, &Size, &Block)>) {
    let blocks: Vec<_> = blocks.iter(world).map(|(entity, pos, size, _)| (*entity, components::to_rect(*pos, *size))).collect();

    for (pos, size, player) in players.iter_mut(world) {
        let block_held = blocks.iter().find(|(e, r)| player.is_holding(*e)).map(|(_, r)| *r);
        let blocks: Vec<_> = blocks.iter().filter(|(e, _)| !player.is_holding(*e)).collect();

        for (entity, block_rect) in blocks {
            let player_rect = components::to_rect(*pos, *size);
            let player_rect = match block_held {
                Some(r) => player_rect.combine_with(r),
                _ => player_rect
            };
            let margin = Vec2::new(player_rect.w * 0.45, player_rect.h * 0.45);

            let collision_info = match check_collision(&player_rect, block_rect, margin) {
                Some(c) => c,
                None => continue
            };

            if collision_info.facing {
                player.face(*entity, block_rect.point() - player_rect.point());
            }

            **pos += collision_info.adjustment;
        }
    }
}

struct CollisionInfo {
    facing: bool,
    adjustment: Vec2,
}

fn check_collision(player_rect: &Rect, block_rect: &Rect, margin: Vec2) -> Option<CollisionInfo> {
    // collision detection with smaller rectangle otherwise player gets stuck for excessive collision due to rounding errors
    if !components::to_inner_rect(*player_rect).overlaps(&components::to_inner_rect(*block_rect)) {
        return None;
    }

    let overlap = match player_rect.intersect(*block_rect) {
        Some(rect) => rect,
        None => return None
    };

    let player_center = components::center(*player_rect);
    let touch_up = overlap.y <= player_center.y;
    let touch_right = overlap.x <= player_center.x;

    let (adjust_x, adjust_y, facing) = if overlap.w < overlap.h {
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
            (adjust_x, adjust_y, false)
        } else if overlap.h >= player_rect.h - margin.y {
            let adjust_y = if player_rect.y <= block_rect.y {
                player_rect.h - overlap.h
            } else {
                overlap.h - player_rect.h
            };
            (adjust_x, adjust_y, true)
        } else {
            (adjust_x, 0.0, false)
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
            (adjust_x, adjust_y, false)
        } else if overlap.w >= player_rect.w - margin.x {
            let adjust_x = if player_rect.x <= block_rect.x {
                player_rect.w - overlap.w
            } else {
                overlap.w - player_rect.w
            };
            (adjust_x, adjust_y, true)
        } else {
            (0.0, adjust_y, false)
        }
    };

    Some(CollisionInfo { facing, adjustment: Vec2::new(adjust_x, adjust_y) })
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
        .add_system(update_positions_system())
        .add_system(draw_sprites_system())
        .add_system(control_player_system())
        .add_system(player_block_collision_system())
        .add_system(held_block_update_system())
        .build();

    while !is_key_down(KeyCode::Escape) {
        clear_background(WHITE);

        schedule.execute(&mut world, &mut resources);

        next_frame().await
    }
}
