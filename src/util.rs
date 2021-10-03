use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use crate::card::{CARD_SCALE, CardComponent, Cards};
use crate::Handles;

/// Returns coordinates for the sprite to be drawn at (`x`; `y`), with a given `z` index.
pub fn xyz(x: f32, y: f32, size: (f32, f32), z_index: f32) -> Vec3 {
    Vec3::new(x + size.0 / 2., y + size.1 / 2., z_index)
}

pub fn cursor_pos(
    window: &Window,
    camera_transform: &Transform,
) -> Option<Vec4> {
    if let Some(pos) = window.cursor_position() {
        let size = Vec2::new(window.width() as f32, window.height() as f32);
        let p = pos - size / 2.0;
        return Some(camera_transform.compute_matrix() * p.extend(0.0).extend(1.0));
    }
    return None;
}

pub fn overlap(
    cursor: Vec4,
    object: Vec3,
    size: (f32, f32),
) -> bool {
    return cursor.x >= object.x - size.0 && cursor.x < object.x + size.0 &&
        cursor.y >= object.y - size.1 && cursor.y < object.y + size.1;
}

pub fn card_transform(x: f32, y: f32) -> Transform {
    return Transform {
        translation: Vec3::new(x, y, 2.),
        scale: Vec3::new(CARD_SCALE, CARD_SCALE, 1.),
        ..Default::default()
    }
}

pub trait Slot {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
}