use bevy::app::{AppBuilder, EventWriter, Plugin};
use bevy::ecs::prelude::*;
use bevy::input::Input;
use bevy::input::mouse::MouseButton;
use bevy::math::{Vec2, Vec4Swizzles};
use bevy::prelude::Transform;
use bevy::window::Windows;
use crate::MainCamera;
use crate::ui::transition::TranslationAnimation;
use crate::util::{cursor_pos, overlap, Z_CARD_DRAG};

pub struct DragAndDropPlugin;

pub struct Draggable {
    pub size: Vec2,
}

pub struct Dragged;
pub struct Dropped(pub Entity);
pub const DROP_BORDER: f32 = 10.;

impl Plugin for DragAndDropPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_event::<Dropped>()
            .add_system(drag_update.system().label("drag:update"))
            .add_system(drop_update.system().label("drag:end").after("drag:update"))
            .add_system(begin_drag.system().label("drag:begin"))
        ;
    }
}

fn drag_update(
    windows: Res<Windows>,
    mut queries: QuerySet<(
        Query<&Transform, With<MainCamera>>,
        Query<&mut Transform, With<Dragged>>,
    )>,
) {
    let window = windows.get_primary().unwrap();
    if let Some(cursor) = cursor_pos(window, queries.q0().single().unwrap()) {
        // Get hovered card id & transform
        for mut transform in queries.q1_mut().iter_mut() {
            transform.translation.x = cursor.x;
            transform.translation.y = cursor.y;
        }
    }
}

fn drop_update(
    mut commands: Commands,
    mut ev_dropped: EventWriter<Dropped>,
    btn: Res<Input<MouseButton>>,
    dragged: Query<Entity, With<Dragged>>,
) {
    if btn.just_released(MouseButton::Left) {
        for e in dragged.iter() {
            commands.entity(e)
                .remove::<Dragged>();
            ev_dropped.send(Dropped(e));
        }
    }
}

fn begin_drag(
    mut commands: Commands,
    btn: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut queries: QuerySet<(
        Query<&Transform, With<MainCamera>>,
        Query<(Entity, &Draggable, &mut Transform), Without<TranslationAnimation>>,
    )>,
) {
    if btn.just_pressed(MouseButton::Left) {
        // Start dragging a card
        let window = windows.get_primary().unwrap();
        if let Some(cursor) = cursor_pos(window, queries.q0().single().unwrap()) {
            // Get hovered card id & transform
            for (e, draggable, mut transform) in queries.q1_mut().iter_mut() {
                if overlap(cursor.xyz(), transform.translation, (draggable.size.x, draggable.size.y)) {
                    commands.entity(e).insert(Dragged);
                    transform.translation.x = cursor.x;
                    transform.translation.y = cursor.y;
                    transform.translation.z = Z_CARD_DRAG;
                    break;
                }
            }
        }
    }
}