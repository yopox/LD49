use bevy::math::{vec2, vec3};
use bevy::prelude::*;

use crate::{AppState, HEIGHT, MySelf, PlayerData, WIDTH};
use crate::card::*;
use crate::Handles;
use crate::ui::{animate, animate_switch, animate_fast, Draggable, Dragged, DROP_BORDER, Dropped, easing, TranslationAnimation};
use crate::util::{card_transform, overlap, Slot};

pub struct ShopPlugin;

/// Cards are in one of these spots
#[derive(PartialEq, Clone, Copy, Debug)]
enum ShopSlots {
    SHOP,
    BOARD,
    HAND,
}

#[derive(PartialEq, Clone)]
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

struct SlotBorder;
struct SlotHovered;

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
                    .with_system(highlight_slot.system())
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

    // Slots
    for i in 0..=6 {
        if i <= 4 {
            commands.spawn().insert(ShopSlot { row: ShopSlots::HAND, id: i });
        }
        commands.spawn().insert(ShopSlot { row: ShopSlots::BOARD, id: i });
        commands.spawn().insert(ShopSlot { row: ShopSlots::SHOP, id: i });
    }

    // Background
    commands.spawn_bundle(SpriteBundle {
        material: handles.shop_bg.clone(),
        transform: Transform {
            translation: Vec3::new(WIDTH / 2., HEIGHT / 2., 0.),
            ..Default::default()
        },
        ..Default::default()
    });

    // Slot border
    commands
        .spawn_bundle(SpriteBundle {
            material: handles.slot_border.clone(),
            visible: Visible {
                is_visible: false,
                is_transparent: true,
            },
            ..Default::default()
        })
        .insert(SlotBorder);
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
        })
        .insert(slot)
    ;
}

fn highlight_slot(
    mut commands: Commands,
    mut queries: QuerySet<(
        Query<(&Transform, &ShopSlot), (With<Card>, With<Dragged>)>,
        Query<(Entity, &ShopSlot), Without<Card>>,
        Query<(&mut Transform, &mut Visible), With<SlotBorder>>,
    )>,
) {
    let dragged_card = queries.q0().single();
    if !dragged_card.is_ok() { return; }
    let (transform, origin_slot) = dragged_card.unwrap();
    let translation = transform.translation.clone();
    let origin_slot = origin_slot.clone();

    let mut hovered_slot: Option<ShopSlot> = None;
    for (e, slot) in queries.q1_mut().iter_mut() {
        if overlap(translation, vec3(slot.x(), slot.y(), 0.),
                   (CARD_WIDTH / 2. + DROP_BORDER, CARD_HEIGHT / 2. + DROP_BORDER)) {
            commands.entity(e).insert(SlotHovered);
            hovered_slot = Some(slot.clone());
        } else {
            commands.entity(e).remove::<SlotHovered>();
        }
    }

    // Check if the card can be dropped
    if hovered_slot.is_none() {
        let (_, mut visible) = queries.q2_mut().single_mut().unwrap();
        visible.is_visible = false;
        return;
    }
    let hovered_slot = hovered_slot.unwrap();
    let possible = match origin_slot.row {
        ShopSlots::SHOP => hovered_slot.row == ShopSlots::HAND,
        ShopSlots::BOARD => hovered_slot.row == ShopSlots::BOARD || hovered_slot.row == ShopSlots::SHOP,
        ShopSlots::HAND => hovered_slot.row == ShopSlots::BOARD,
    };

    let (mut border_transform, mut visible) = queries.q2_mut().single_mut().unwrap();
    if possible {
        visible.is_visible = true;
        border_transform.translation.x = hovered_slot.x();
        border_transform.translation.y = hovered_slot.y();
    } else {
        visible.is_visible = false;
    }
}

fn drop_card(
    mut commands: Commands,
    mut ev_dropped: EventReader<Dropped>,
    time: Res<Time>,
    mut queries: QuerySet<(
        Query<(Entity, &ShopSlot), With<SlotHovered>>,
        Query<&mut Visible, With<SlotBorder>>,
        Query<(Entity, &Transform, &mut ShopSlot), With<Card>>,
    )>,
) {
    for dropped in ev_dropped.iter() {
        // Get hovered slot and remove SlotHovered component
        let hovered_slot: Option<ShopSlot> = match queries.q0_mut().single_mut() {
            Ok((e, slot)) => {
                commands.entity(e).remove::<SlotHovered>();
                Some(slot.clone())
            }
            Err(_) => None
        };

        match hovered_slot {
            None => {
                for (e, transform, mut slot) in queries.q2_mut().iter_mut() {
                    if dropped.0 == e {
                        println!("No slots hovered. Fallback to {:?} {}", &slot.row, &slot.id);
                        commands
                            .entity(dropped.0)
                            .insert(animate(&time, (transform.translation.x, transform.translation.y), (slot.x(), slot.y())));
                    }
                }
            }
            Some(destination_slot) => {
                // Hide border
                queries.q1_mut().single_mut().unwrap().is_visible = false;

                let mut origin_slot: Option<ShopSlot> = None;
                let mut existing_entity: Option<Entity> = None;
                for (e, _, slot) in queries.q2_mut().iter_mut() {
                    if dropped.0 == e {
                        origin_slot = Some(slot.clone());
                    } else if destination_slot == *slot {
                        existing_entity = Some(e.clone());
                    }
                }

                let origin_slot = origin_slot.unwrap();
                let origin_pos = (origin_slot.x(), origin_slot.y());

                // TODO: Illegal to buy if the player has not enough gold
                let legal_move: bool = match origin_slot.row {
                    ShopSlots::HAND => destination_slot.row == ShopSlots::BOARD && existing_entity.is_none(),
                    ShopSlots::BOARD => destination_slot.row == ShopSlots::BOARD || destination_slot.row == ShopSlots::SHOP,
                    ShopSlots::SHOP => destination_slot.row == ShopSlots::HAND && existing_entity.is_none(),
                };
                println!["Move: {:?} {} -> {:?} {} : {}", &origin_slot.row, &origin_slot.id, &destination_slot.row, &destination_slot.id, legal_move];

                for (e, transform, mut slot) in queries.q2_mut().iter_mut() {
                    if dropped.0 == e {
                        if legal_move {
                            slot.row = destination_slot.row;
                            slot.id = destination_slot.id;
                            commands
                                .entity(e)
                                .insert(animate_fast(&time, (transform.translation.x, transform.translation.y), (destination_slot.x(), destination_slot.y())));
                        } else {
                            commands
                                .entity(e)
                                .insert(animate(&time, (transform.translation.x, transform.translation.y), (origin_pos.0, origin_pos.1)));
                        }
                    }
                }

                if legal_move {
                    if let Some(existing_entity) = existing_entity {
                        for (e, transform, mut slot) in queries.q2_mut().iter_mut() {
                            if existing_entity == e {
                                slot.row = origin_slot.row;
                                slot.id = origin_slot.id;
                                commands
                                    .entity(e)
                                    .insert(animate_switch(&time, (transform.translation.x, transform.translation.y), (origin_pos.0, origin_pos.1)));
                            }
                        }
                    }
                }
            }
        }
    }
}