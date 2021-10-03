use bevy::math::{vec2, vec3};
use bevy::prelude::*;

use crate::{AppState, HEIGHT, MySelf, PlayerData, WIDTH};
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
            ShopSlots::SHOP => 192. + 128. * self.id as f32,
            ShopSlots::BOARD => 256. + 128. * self.id as f32,
            ShopSlots::HAND => 448. + 128. * self.id as f32,
        }
    }

    fn y(&self) -> f32 {
        match &self.row {
            ShopSlots::SHOP => HEIGHT - 160.,
            ShopSlots::BOARD => HEIGHT - 384.,
            ShopSlots::HAND => HEIGHT - 576.,
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
    query: Query<&PlayerData, With<MySelf>>,
) {
    let player_data = query.single().expect(
        "There should be one and only one player with myself"
    );

    for (i, &card) in player_data.board.iter().enumerate() {
        add_card(card,
                 ShopSlot { row: ShopSlots::BOARD, id: i as u8 },
                 &mut commands, &handles);
    }

    for (i, &card) in player_data.hand.iter().enumerate() {
        add_card(card,
                 ShopSlot { row: ShopSlots::HAND, id: i as u8 },
                 &mut commands, &handles);
    }

    add_card(Card::from(CardsID::MERCH_8),
             ShopSlot { row: ShopSlots::SHOP, id: 0 },
             &mut commands, &handles);

    add_card(Card::from(CardsID::MUSH_8),
             ShopSlot { row: ShopSlots::SHOP, id: 1 },
             &mut commands, &handles);

    commands.spawn_bundle(SpriteBundle {
        material: handles.shop_bg.clone(),
        transform: Transform {
            translation: Vec3::new(WIDTH / 2., HEIGHT / 2., 0.),
            ..Default::default()
        },
        ..Default::default()
    });
}

fn add_card(card: Card, slot: ShopSlot, commands: &mut Commands, handles: &Res<Handles>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: card.card_id.handle(&handles),
            transform: card_transform(slot.x(), slot.y()),
            ..Default::default()
        })
        .insert(card)
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
    query: Query<(Entity, &ShopSlot, &Dropped), With<Card>>,
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