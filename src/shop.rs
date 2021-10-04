use std::cmp::{min, max};
use bevy::math::{vec2, vec3};
use bevy::prelude::*;

use crate::{AppState, HEIGHT, MySelf, PlayerData, WIDTH};
use crate::card::*;
use crate::font::TextStyles;
use crate::GlobalData;
use crate::Handles;
use crate::ui::{animate, animate_switch, animate_fast, Draggable, Dragged, DROP_BORDER, Dropped, easing, TranslationAnimation, TransitionOver, StateBackground};
use crate::util::{card_transform, cleanup_system, overlap, Slot, Z_BACKGROUND, Z_BOB};

pub struct ShopPlugin;
pub struct Coins;
pub struct Level;

/// Cards are in one of these spots
#[derive(PartialEq, Clone, Copy, Debug)]
enum ShopSlots {
    SHOP,
    BOARD,
    HAND,
    SELL,
}

#[derive(PartialEq, Clone)]
struct ShopSlot {
    row: ShopSlots,
    id: u8,
}

impl Slot for ShopSlot {
    fn x(&self) -> f32 {
        match &self.row {
            ShopSlots::SHOP => 192. + 128. * (self.id + if self.id > 2 { 1 } else { 0 }) as f32,
            ShopSlots::SELL => 192. + 128. * 3.,
            ShopSlots::BOARD => 256. + 128. * self.id as f32,
            ShopSlots::HAND => 448. + 128. * self.id as f32,
        }
    }

    fn y(&self) -> f32 {
        match &self.row {
            ShopSlots::SHOP => HEIGHT - 160.,
            ShopSlots::SELL => HEIGHT - 160.,
            ShopSlots::BOARD => HEIGHT - 384.,
            ShopSlots::HAND => HEIGHT - 576.,
        }
    }
}

struct Bob;
struct SlotBorder;
struct SlotHovered;
struct Sold;
struct CoinsDiff(i8, bool); // (gained coins ; can overflow)
struct CoinLimit(u16);
struct BeganShop(f64);
const MIN_COINS: u16 = 3;

struct ShopCosts {
    buy: i8,
    sell: i8,
    refresh: i8,
    freeze: i8,
    gold_limit: u16,
}

impl Default for ShopCosts {
    fn default() -> Self {
        ShopCosts {
            buy: 3,
            sell: -1,
            refresh: 1,
            freeze: 0,
            gold_limit: 10,
        }
    }
}

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_event::<CoinsDiff>()
            .add_system_set(
                SystemSet::on_enter(AppState::Shop)
                    .with_system(init.system().label("shop:init"))
            )
            .add_system_set(
                SystemSet::on_update(AppState::Shop)
                    .after("shop:init")
                    .with_system(drop_card.system().label("drop").after("drag:end"))
                    .with_system(highlight_slot.system().after("drag:update"))
                    .with_system(update_ui.system())
                    .with_system(sell_card.system().after("drop"))
                    .with_system(update_coins.system())
            )
            .add_system_set(
                SystemSet::on_exit(AppState::Shop)
                    .label("on_exit")
                    .with_system(on_exit.system())
            )
            .add_system_set(
                SystemSet::on_exit(AppState::Shop)
                    .after("on_exit")
                    .with_system(cleanup_system::<ShopSlot>.system())
                    .with_system(cleanup_system::<SlotBorder>.system())
                    .with_system(cleanup_system::<Bob>.system())
                    .with_system(cleanup_system::<ShopCosts>.system())
                    .with_system(cleanup_system::<CoinLimit>.system())
                    .with_system(cleanup_system::<Level>.system())
                    .with_system(cleanup_system::<Coins>.system())
                    .with_system(cleanup_system::<BeganShop>.system())
                    .with_system(cleanup_system::<StateBackground>.system())
            )
        ;
    }
}

fn init(
    time: Res<Time>,
    mut commands: Commands,
    mut global_data: ResMut<GlobalData>,
    handles: Res<Handles>,
    text_styles: Res<TextStyles>,
    query: Query<&PlayerData, With<MySelf>>,
) {
    let player_data = query.single().expect(
        "There should be one and only one player with myself"
    );

    let costs = ShopCosts::default();
    commands.insert_resource(CoinLimit(max(MIN_COINS, min(global_data.turn, costs.gold_limit))));
    commands.insert_resource(costs);

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

    add_card(Card::new(CardTypes::MERCH_8, global_data.next_card_id),
             ShopSlot { row: ShopSlots::SHOP, id: 0 },
             &mut commands, &handles);
    global_data.next_card_id += 1;

    add_card(Card::new(CardTypes::MUSH_8, global_data.next_card_id),
             ShopSlot { row: ShopSlots::SHOP, id: 1 },
             &mut commands, &handles);
    global_data.next_card_id += 1;

    // Slots
    for i in 0..=6 {
        if i == 0 {
            commands.spawn().insert(ShopSlot { row: ShopSlots::SELL, id: 0 });
        }
        if i <= 4 {
            commands.spawn().insert(ShopSlot { row: ShopSlots::HAND, id: i });
        }
        if i < 6 {
            commands.spawn().insert(ShopSlot { row: ShopSlots::SHOP, id: i });
        }
        commands.spawn().insert(ShopSlot { row: ShopSlots::BOARD, id: i });
    }

    // Background
    commands.spawn_bundle(SpriteBundle {
        material: handles.shop_bg.clone(),
        transform: Transform {
            translation: Vec3::new(WIDTH / 2., HEIGHT / 2., Z_BACKGROUND),
            ..Default::default()
        },
        ..Default::default()
    }).insert(StateBackground);

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

    // UI
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(15.0),
                    left: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text::with_section(
                "".to_string(),
                text_styles.love_bug_small.clone(),
                Default::default()
            ),
            transform: Default::default(),
            ..Default::default()
        })
        .insert(Coins);

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(15.0),
                    left: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: format!("TURN {}\n", global_data.turn),
                        style: text_styles.love_bug_small.clone(),
                        ..Default::default()
                    },
                    TextSection {
                        value: "".to_string(),
                        style: text_styles.love_bug_small.clone(),
                        ..Default::default()
                    }
                ],
              ..Default::default()
            },
            transform: Default::default(),
            ..Default::default()
        })
        .insert(Level);

    commands.spawn_bundle(TextBundle {
        style: Style {
            // align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: Rect {
                top: Val::Px(15.0),
                right: Val::Px(15.0),
                ..Default::default()
            },
            ..Default::default()
        },
        text: Text {
            sections: vec![
                TextSection {
                    value: format!("REMAINING TIME 60s\n"),
                    style: text_styles.love_bug_small.clone(),
                    ..Default::default()
                },
            ],
            ..Default::default()
        },
        transform: Default::default(),
        ..Default::default()
    }).insert(BeganShop(time.seconds_since_startup()));
}

fn add_card(card: Card, slot: ShopSlot, commands: &mut Commands, handles: &Res<Handles>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: card.card_type.handle(&handles),
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

fn update_ui(
    time: Res<Time>,
    coin_limit: Res<CoinLimit>,
    mut state: ResMut<State<AppState>>,
    mut queries: QuerySet<(
        Query<&PlayerData, With<MySelf>>,
        Query<&mut Text, With<Coins>>,
        Query<&mut Text, With<Level>>,
        Query<(&mut Text, &BeganShop)>,
    )>,
) {
    let data = queries.q0().single().expect("No data for the player");
    let coins = data.coins;
    let level = data.shop_level;

    let mut coins_text = queries.q1_mut().single_mut().expect("Coins text not found.");
    coins_text.sections[0].value = format!("COINS: {}/{}", coins, coin_limit.0);

    let mut level_text = queries.q2_mut().single_mut().expect("Level text not found.");
    level_text.sections[1].value = format!("SHOP LEVEL {}", level);

    let (mut time_text, BeganShop(t0)) = queries.q3_mut().single_mut().expect("TIme text not found.");
    let remaining_time = 10. - time.seconds_since_startup() + *t0;
    time_text.sections[0].value = format!("REMAINING TIME {}s", remaining_time as u8);

    if remaining_time < 0. {
        state.set(AppState::Fight);
    }

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
    if !dragged_card.is_ok() {
        // No card is dragged => hide the border
        let (_, mut visible) = queries.q2_mut().single_mut().unwrap();
        visible.is_visible = false;
        return;
    }
    let (transform, origin_slot) = dragged_card.unwrap();
    let translation = transform.translation.clone();
    let origin_slot = origin_slot.clone();

    // Update hovered slot
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

    // Check if the card can be dropped on this row
    if hovered_slot.is_none() {
        let (_, mut visible) = queries.q2_mut().single_mut().unwrap();
        visible.is_visible = false;
        return;
    }
    let hovered_slot = hovered_slot.unwrap();
    let possible = match origin_slot.row {
        ShopSlots::SHOP => hovered_slot.row == ShopSlots::HAND,
        ShopSlots::BOARD => hovered_slot.row == ShopSlots::BOARD || hovered_slot.row == ShopSlots::SELL,
        ShopSlots::HAND => hovered_slot.row == ShopSlots::BOARD || hovered_slot.row == ShopSlots::HAND,
        ShopSlots::SELL => false,
    };

    // Update the border accordingly
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
    mut ev_coins: EventWriter<CoinsDiff>,
    costs: Res<ShopCosts>,
    time: Res<Time>,
    mut queries: QuerySet<(
        Query<(Entity, &ShopSlot), With<SlotHovered>>,
        Query<(Entity, &Transform, &mut ShopSlot), With<Card>>,
        Query<(&PlayerData), With<MySelf>>,
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
                // Find the dragged card and send it back to its slot
                for (e, transform, mut slot) in queries.q1_mut().iter_mut() {
                    if dropped.0 == e {
                        // println!("No slots hovered. Fallback to {:?} {}", &slot.row, &slot.id);
                        commands
                            .entity(dropped.0)
                            .insert(animate(&time, (transform.translation.x, transform.translation.y), (slot.x(), slot.y())));
                    }
                }
            }
            Some(destination_slot) => {
                // Get the slot where the card is dragged from
                // Check if there is a card on the destination
                let mut origin_slot: Option<ShopSlot> = None;
                let mut existing_entity: Option<Entity> = None;
                for (e, _, slot) in queries.q1_mut().iter_mut() {
                    if dropped.0 == e {
                        origin_slot = Some(slot.clone());
                    } else if destination_slot == *slot {
                        existing_entity = Some(e);
                    }
                }

                let origin_slot = origin_slot.unwrap();
                let origin_pos = (origin_slot.x(), origin_slot.y());

                let (data) = queries.q2().single().expect("Can't find player data.");

                let legal_move: bool = match origin_slot.row {
                    ShopSlots::HAND => destination_slot.row == ShopSlots::HAND || destination_slot.row == ShopSlots::BOARD && existing_entity.is_none(),
                    ShopSlots::BOARD => destination_slot.row == ShopSlots::BOARD || destination_slot.row == ShopSlots::SELL,
                    ShopSlots::SHOP => destination_slot.row == ShopSlots::HAND && existing_entity.is_none() && data.coins >= costs.buy as u16,
                    ShopSlots::SELL => false,
                };
                // println!["Move: {:?} {} -> {:?} {} : {}", &origin_slot.row, &origin_slot.id, &destination_slot.row, &destination_slot.id, legal_move];

                // Move the dragged card to its new slot (or old slot if the move isn't legal)
                for (e, transform, mut slot) in queries.q1_mut().iter_mut() {
                    if dropped.0 == e {
                        if legal_move {
                            slot.row = destination_slot.row;
                            slot.id = destination_slot.id;
                            commands
                                .entity(e)
                                .insert(animate_fast(&time, (transform.translation.x, transform.translation.y), (destination_slot.x(), destination_slot.y())));
                            if destination_slot.row == ShopSlots::SELL { commands.entity(e).insert(Sold); }
                            if origin_slot.row == ShopSlots::SHOP && destination_slot.row == ShopSlots::HAND {
                                ev_coins.send(CoinsDiff(costs.buy, false));
                            }
                        } else {
                            commands
                                .entity(e)
                                .insert(animate(&time, (transform.translation.x, transform.translation.y), (origin_pos.0, origin_pos.1)));
                        }
                    }
                }

                // Move the card already on the destination to the origin slot
                // If we sell, ignore existing_entity
                if legal_move && destination_slot.row != ShopSlots::SHOP {
                    if let Some(existing_entity) = existing_entity {
                        for (e, transform, mut slot) in queries.q1_mut().iter_mut() {
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

fn sell_card(
    mut commands: Commands,
    costs: Res<ShopCosts>,
    mut ev_transition: EventReader<TransitionOver>,
    mut ev_coins: EventWriter<CoinsDiff>,
    mut cards: Query<(Entity, &ShopSlot), With<Card>>,
) {
    for transition in ev_transition.iter() {
        for (e, slot) in cards.iter_mut() {
            if e == transition.0 && slot.row == ShopSlots::SELL {
                commands.entity(transition.0).despawn();
                ev_coins.send(CoinsDiff(costs.sell, false));
            }
        }
    }
}

fn update_coins(
    coin_limit: Res<CoinLimit>,
    mut ev_coins: EventReader<CoinsDiff>,
    mut data: Query<&mut PlayerData, With<MySelf>>,
) {
    for diff in ev_coins.iter() {
        let (mut player_data) = data.single_mut().expect("Can't find player data.");
        if !diff.1 && diff.0 < 0 && player_data.coins + (-diff.0) as u16 > coin_limit.0 { break; }
        player_data.coins = (player_data.coins as i16 - diff.0 as i16) as u16;
    }
}

fn on_exit(
    mut commands: Commands,
    cards: Query<(Entity, &Card, &ShopSlot)>,
    mut player_data: Query<&mut PlayerData, With<MySelf>>,
) {
    let mut player_data = player_data.single_mut().expect("There should only be one player tagged with myself");
    player_data.board = vec![];
    player_data.hand = vec![];
    for (e, &card, slot) in cards.iter() {
        match slot.row {
            ShopSlots::BOARD => {
                player_data.board.push(card);
            }
            ShopSlots::HAND => {
                player_data.hand.push(card);
            }
            ShopSlots::SHOP => {}
            ShopSlots::SELL => {}
        };
        commands.entity(e).despawn_recursive();
    }
}