use legion::{systems::Builder, *};
use macroquad::prelude::*;

use crate::components::{self, Position, Size};

#[derive(Debug)]
pub struct Camera;

#[system(for_each)]
fn update_camera(position: &Position, size: &Size, _: &Camera) {
    let center = components::center(components::to_rect(*position, *size));
    let offset_x = screen_width() / 2.0;
    let offset_y = screen_height() / 2.0;
    let camera = Camera2D::from_display_rect(Rect::new(
        center.x - offset_x,
        center.y - offset_y,
        offset_x * 2.0,
        offset_y * 2.0,
    ));

    set_camera(&camera);
}

pub fn setup_systems(builder: &mut Builder) -> &mut Builder {
    builder.add_system(update_camera_system())
}
