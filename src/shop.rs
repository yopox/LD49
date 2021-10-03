use bevy::math::{vec2, vec3};
use bevy::prelude::*;

use crate::AppState;
use crate::card::*;
use crate::Handles;
use crate::ui::{Draggable, Dropped, easing, TranslationAnimation};
use crate::util::{card_transform, Slot};

pub struct ShopPlugin;

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
        .insert(Draggable {
            size: vec2(CARD_WIDTH / 2., CARD_HEIGHT / 2.),
            pos: vec3(slot.x(), slot.y(), 0.0),
        })
        .insert(slot)
    ;
}

fn drop_card(
    mut commands: Commands,
    time: Res<Time>,
    query: Query<(Entity, &ShopSlot, &Dropped), With<CardComponent>>,
) {
    for (e, slot, dropped) in query.iter() {
        commands.entity(e)
            .remove::<Dropped>()
            .insert(TranslationAnimation::from_start_end(
                time.seconds_since_startup(),
                1.3,
                vec3(dropped.position.x, dropped.position.y, 0.),
                vec3(slot.x(), slot.y(), 0.),
                easing::Functions::CubicOut,
            ));
    }
}