use bevy::prelude::*;

use crate::{AppState, HEIGHT, MainCamera, WIDTH};
use crate::card::*;
use crate::Handles;
use crate::util::{card_transform, cursor_pos, overlap, Slot};

pub struct ShopPlugin;

struct Dragged;

/// Cards are in one of these spots
enum ShopSlots {
    SHOP,
    BOARD,
    HAND,
}

struct ShopSlot {
    row: ShopSlots,
    id: u8,
}

impl Slot for ShopSlot {
    fn x(&self) -> f32 {
        match &self.row {
            ShopSlots::SHOP => 300. + 200. * self.id as f32,
            ShopSlots::BOARD => 300. + 200. * self.id as f32,
            ShopSlots::HAND => 300. + 200. * self.id as f32,
        }
    }

    fn y(&self) -> f32 {
        match &self.row {
            ShopSlots::SHOP => 625.,
            ShopSlots::BOARD => 375.,
            ShopSlots::HAND => 125.,
        }
    }
}

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
    add_card(Cards::SPID_8,
             ShopSlot { row: ShopSlots::HAND, id: 0 },
             &mut commands, &handles);

    add_card(Cards::ROB_8,
             ShopSlot { row: ShopSlots::BOARD, id: 0 },
             &mut commands, &handles);

    add_card(Cards::MERCH_8,
             ShopSlot { row: ShopSlots::SHOP, id: 0 },
             &mut commands, &handles);

    add_card(Cards::MUSH_8,
             ShopSlot { row: ShopSlots::SHOP, id: 1 },
             &mut commands, &handles);
}

fn add_card(id: Cards, slot: ShopSlot, commands: &mut Commands, handles: &Res<Handles>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: id.handle(&handles),
            transform: card_transform(slot.x(), slot.y()),
            ..Default::default()
        })
        .insert(CardComponent { card_id: id })
        .insert(slot);
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