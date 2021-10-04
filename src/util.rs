use bevy::ecs::component::Component;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use crate::card::{Card, CARD_SCALE, BaseCards};
use crate::font::TextStyles;

pub struct Coins;

pub struct Level;

pub struct PlayerHP;

pub const Z_BACKGROUND: f32 = 0.;
pub const Z_STATS: f32 = 0.1; // (+ 10., children of card)
pub const Z_CARD: f32 = 10.;
pub const Z_ABILITY: f32 = 15.;
pub const Z_CARD_SWITCH: f32 = 20.;
pub const Z_CARD_DRAG: f32 = 25.;
pub const Z_BOB: f32 = 100.;
pub const Z_POPUP_BG: f32 = 120.;
pub const Z_POPUP_TEXT: f32 = 121.;
pub const Z_ANNOUNCEMENT_BG: f32 = 130.;
pub const Z_ANNOUNCEMENT_TEXT: f32 = 131.;


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
    cursor: Vec3,
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
    };
}

pub trait Slot {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
}

pub fn cleanup_system<T: Component>(
    mut commands: Commands,
    q: Query<Entity, With<T>>,
) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}

pub enum Corners {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

pub fn text_bundle_at_corner(corner: Corners, values: Vec<String>, style: &TextStyle) -> TextBundle {
    let dist = Val::Px(32.);
    let position = match corner {
        Corners::TopLeft => Rect { top: dist, left: dist, ..Default::default() },
        Corners::TopRight => Rect { top: dist, right: dist, ..Default::default() },
        Corners::BottomLeft => Rect { bottom: dist, left: dist, ..Default::default() },
        Corners::BottomRight => Rect { bottom: dist, right: dist, ..Default::default() },
    };
    let sections: Vec<TextSection> = values.into_iter().map(|value|
        TextSection {
            value,
            style: style.clone(),
            ..Default::default()
        }).collect();
    TextBundle {
        style: Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position,
            ..Default::default()
        },
        text: Text {
            sections,
            ..Default::default()
        },
        transform: Default::default(),
        ..Default::default()
    }
}