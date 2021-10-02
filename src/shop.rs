use bevy::prelude::*;

use crate::{AppState, HEIGHT, MainCamera, WIDTH};
use crate::card::*;
use crate::Handles;
use crate::util::{cursor_pos, overlap};

pub struct ShopPlugin;
struct  Dragged;

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system_set(
                SystemSet::on_enter(AppState::Shop)
                    .with_system(init.system())
            )
            .add_system_set(
                SystemSet::on_update(AppState::Shop)
                    .with_system(drag_card.system())
                    .with_system(move_card.system())
                    .with_system(drop_card.system())
            );
    }
}

fn init(
    mut commands: Commands,
    handles: Res<Handles>,
) {
    commands
        .spawn_bundle(SpriteBundle {
            material: handles.dummy_card.clone(),
            transform: card_transform(WIDTH / 2. - 200., HEIGHT / 2.),
            ..Default::default()
        })
        .insert(CardComponent { card_id: Cards::DUMMY_1 });

    commands
        .spawn_bundle(SpriteBundle {
            material: handles.dummy_card2.clone(),
            transform: card_transform(WIDTH / 2. + 100., HEIGHT / 2.),
            ..Default::default()
        })
        .insert(CardComponent { card_id: Cards::DUMMY_2 });
}

fn drag_card(
    mut commands: Commands,
    btn: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    queries: QuerySet<(
        Query<&Transform, With<MainCamera>>,
        Query<(Entity, &Transform), With<CardComponent>>,
    )>,
) {
    if btn.just_pressed(MouseButton::Left) {
        // Start dragging a card
        let mut entity: Option<Entity> = None;
        let window = windows.get_primary().unwrap();
        if let Some(cursor) = cursor_pos(window, queries.q0().single().unwrap()) {
            // Get hovered card id & transform
            for (e, transform) in queries.q1().iter() {
                let card_pos = transform.translation;
                if overlap(cursor, card_pos, (CARD_WIDTH / 2., CARD_HEIGHT / 2.)) {
                    entity = Some(e.clone());
                    break;
                }
            }
        }
        if let Some(card) = entity {
            commands
                .entity(card)
                .insert(Dragged);
        }
    }
}

fn move_card(
    windows: Res<Windows>,
    mut queries: QuerySet<(
        Query<&Transform, With<MainCamera>>,
        Query<(&mut Transform), With<Dragged>>,
    )>,
) {
    let window = windows.get_primary().unwrap();
    if let Some(cursor) = cursor_pos(window, queries.q0().single().unwrap()) {
        // Get hovered card id & transform
        for (mut transform) in queries.q1_mut().iter_mut() {
            transform.translation.x = cursor.x;
            transform.translation.y = cursor.y;
        }
    }
}

fn drop_card(
    mut commands: Commands,
    btn: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    queries: QuerySet<(
        Query<&Transform, With<MainCamera>>,
        Query<(Entity, &Transform), (With<CardComponent>, With<Dragged>)>,
    )>,
) {
    if btn.just_released(MouseButton::Left) {
        // Drop the card
        let mut entity: Option<Entity> = None;
        let window = windows.get_primary().unwrap();
        if let Some(cursor) = cursor_pos(window, queries.q0().single().unwrap()) {
            // Get hovered card id & transform
            for (e, transform) in queries.q1().iter() {
                let card_pos = transform.translation;
                if overlap(cursor, card_pos, (CARD_WIDTH / 2., CARD_HEIGHT / 2.)) {
                    entity = Some(e.clone());
                    break;
                }
            }
        }
        if let Some(card) = entity {
            commands
                .entity(card)
                .remove::<Dragged>();
        }
    }
}