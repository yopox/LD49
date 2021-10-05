use std::cmp::{max, min};
use std::collections::HashSet;

use bevy::math::{vec2, vec3, Vec4Swizzles};
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel, AudioPlugin};
use rand::Rng;
use rand::prelude::SliceRandom;

use crate::{AppState, HEIGHT, MainCamera, MySelf, PlayerData, WIDTH};
use crate::card::*;
use crate::fight::MyFoe;
use crate::font::TextStyles;
use crate::GlobalData;
use crate::loading::{AudioAssets, ColorAssets};
use crate::loading::TextureAssets;
use crate::shop_manager::ShopManager;
use crate::shop_rules::ShopRules;
use crate::ui::{animate, animate_fast, animate_switch, DisplayBetweenAnimation, Draggable, Dragged, DROP_BORDER, Dropped, easing, RemoveAfter, StateBackground, TransitionOver, TranslationAnimation};
use crate::util::{card_transform, cleanup_system, Coins, Corners, cursor_pos, Level, overlap, PlayerHP, Slot, text_bundle_at_corner, Z_ABILITY, Z_ANNOUNCEMENT_BG, Z_BACKGROUND, Z_BOB, Z_POPUP_BG, Z_POPUP_TEXT};

pub struct ShopPlugin;

/// Cards are in one of these spots
#[derive(PartialEq, Clone, Copy, Debug)]
enum ShopSlots {
    SHOP,
    BOARD,
    HAND,
    SELL,
}

#[derive(PartialEq, Copy, Clone)]
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

struct ShopUi;

struct RefreshButton;

struct FreezeButton;

struct UpgradeButton;

struct ButtonText;

struct Sold;

// (gained coins ; can overflow)
struct CoinsDiff(i8, bool);

struct CoinLimit(u16);

struct BeganShop(f64);

struct PlayedTrigger(Entity);

struct SoldTrigger(Card);

const MIN_COINS: u16 = 3;

pub struct ShopValues {
    pub buy: i8,
    pub sell: i8,
    pub refresh: u16,
    pub freeze: u16,
    pub gold_limit: u16,
    pub timer: f64,
}

impl Default for ShopValues {
    fn default() -> Self {
        ShopValues {
            buy: 3,
            sell: -1,
            refresh: 1,
            freeze: 0,
            gold_limit: 10,
            timer: 40.,
        }
    }
}

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_event::<CoinsDiff>()
            .add_event::<PlayedTrigger>()
            .add_event::<SoldTrigger>()
            .insert_resource(ShopFrozen(None))
            .insert_resource(CanRefresh(false))
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
                    .with_system(start_draggable.system())
                    .with_system(display_ability_animation.system())
                    .with_system(handle_buttons.system())
                    .with_system(played_trigger.system())
                    .with_system(sold_trigger.system())
            )
            .add_system_set(
                SystemSet::on_exit(AppState::Shop)
                    .label("on_exit")
                    .with_system(on_exit.system())
            )
            .add_system_set(
                SystemSet::on_exit(AppState::Shop)
                    .label("shop:cleanup")
                    .after("on_exit")
                    .with_system(cleanup_system::<ShopSlot>.system())
                    .with_system(cleanup_system::<SlotBorder>.system())
                    .with_system(cleanup_system::<ShopUi>.system())
                    .with_system(cleanup_system::<ShopValues>.system())
                    .with_system(cleanup_system::<CoinLimit>.system())
                    .with_system(cleanup_system::<Level>.system())
                    .with_system(cleanup_system::<Coins>.system())
                    .with_system(cleanup_system::<PlayerHP>.system())
                    .with_system(cleanup_system::<BeganShop>.system())
                    .with_system(cleanup_system::<StateBackground>.system())
            )
        ;
    }
}

struct ShopFrozen(Option<Vec<(u8, Card)>>);

struct CanRefresh(bool);

const SHOP_RULE_POPUP_DURATION: f64 = 8.;


fn init(
    time: Res<Time>,
    mut commands: Commands,
    mut global_data: ResMut<GlobalData>,
    mut ev_new_card: EventWriter<NewCard>,
    handles: Res<TextureAssets>,
    text_styles: Res<TextStyles>,
    colors: Res<ColorAssets>,
    audio: Res<Audio>,
    songs: Res<AudioAssets>,
    mut query: Query<&mut PlayerData, With<MySelf>>,
    mut frozen_shop: ResMut<ShopFrozen>,
) {
    let mut player_data = query.single_mut().expect(
        "There should be one and only one player with myself"
    );

    global_data.turn += 1;

    audio.stop();
    audio.play_looped_with_intro(songs.intro.clone(), songs.shop.clone());

    let mut shop_values = ShopValues::default();
    let rule = ShopRules::random(&mut shop_values, global_data.turn);

    let coins = max(MIN_COINS, min(global_data.turn + 2, shop_values.gold_limit))
        + player_data.extra_coins;
    player_data.coins = coins;
    player_data.extra_coins = 0;
    commands.insert_resource(CoinLimit(coins));
    commands.insert_resource(shop_values);

    let t0 = time.seconds_since_startup();

    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section(
            format!("You find a note on the door of the shop:\n\n{}", rule),
            text_styles.note.clone(),
            TextAlignment {
                horizontal: HorizontalAlign::Center,
                vertical: VerticalAlign::Center,
            }),
        transform: Transform {
            translation: Vec3::new(WIDTH / 2., HEIGHT / 2., Z_ANNOUNCEMENT_BG),
            ..Default::default()
        },
        ..Default::default()
    }).insert(RemoveAfter(t0 + SHOP_RULE_POPUP_DURATION));
    commands.spawn_bundle(SpriteBundle {
        material: colors.black.clone(),
        sprite: Sprite::new(Vec2::new(WIDTH / 1.5, HEIGHT / 2.)),
        visible: Visible {
            is_visible: true,
            is_transparent: false,
        },
        transform: Transform {
            translation: Vec3::new(WIDTH / 2., HEIGHT / 2., Z_ANNOUNCEMENT_BG),
            ..Default::default()
        },
        ..Default::default()
    }).insert(RemoveAfter(t0 + SHOP_RULE_POPUP_DURATION));

    commands.spawn().insert(AbilitiesStack {
        next_tick_after: t0 + SHOP_RULE_POPUP_DURATION + 0.5,
        stack: player_data.board.iter()
            .filter(|card| card.base_card.trigger() == Triggers::Turn)
            .map(|card| (card.base_card.ability(), card.id))
            .collect(),
    });

    for (i, &card) in player_data.board.iter().enumerate() {
        add_card(card,
                 ShopSlot { row: ShopSlots::BOARD, id: i as u8 },
                 &mut commands, &handles, &mut ev_new_card);
    }

    for (i, &card) in player_data.hand.iter().enumerate() {
        add_card(card,
                 ShopSlot { row: ShopSlots::HAND, id: i as u8 },
                 &mut commands, &handles, &mut ev_new_card);
    }

    let to_display_in_shop =
        if frozen_shop.0.is_some() {
            let v = frozen_shop.0.as_ref().unwrap().clone();
            *frozen_shop = ShopFrozen(None);
            v
        } else {
            ShopManager::shop_inventory(player_data.shop_level, &mut global_data.rng)
                .iter().enumerate().map(|(i, &base_card)| {
                let id = global_data.next_card_id;
                global_data.next_card_id += 1;
                (i as u8, Card::new(base_card, id))
            }).collect()
        };
    for (i, card) in to_display_in_shop {
        add_card(card,
                 ShopSlot { row: ShopSlots::SHOP, id: i as u8 },
                 &mut commands, &handles, &mut ev_new_card);
    }

    let bob_slot = ShopSlot { row: ShopSlots::SELL, id: 0 };
    commands
        .spawn_bundle(SpriteBundle {
            material: handles.shop_bob.clone(),
            transform: Transform {
                translation: Vec3::new(bob_slot.x(), bob_slot.y(), Z_BOB),
                scale: Vec3::new(CARD_SCALE, CARD_SCALE, 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Bob)
        .insert(ShopUi);

    // Slots
    commands.spawn().insert(bob_slot);
    for i in 0..=6 {
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
        .spawn_bundle(
            text_bundle_at_corner(
                Corners::BottomLeft,
                vec!["".to_string()],
                &text_styles.love_bug_small,
            )
        )
        .insert(Coins);

    commands
        .spawn_bundle(
            text_bundle_at_corner(
                Corners::BottomRight,
                vec![format!("YOUR HP {}", player_data.hp)],
                &text_styles.love_bug_small,
            )
        )
        .insert(PlayerHP);

    commands
        .spawn_bundle(
            text_bundle_at_corner(
                Corners::TopLeft,
                vec![format!("TURN {}\n", global_data.turn), "".to_string()],
                &text_styles.love_bug_small,
            )
        )
        .insert(Level);

    commands
        .spawn_bundle(SpriteBundle {
            material: handles.refresh_button.clone(),
            transform: Transform {
                translation: Vec3::new(1155., HEIGHT / 2. + 140., Z_BOB),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RefreshButton)
        .insert(ShopUi);

    commands
        .spawn_bundle(SpriteBundle {
            material: handles.freeze_button.clone(),
            transform: Transform {
                translation: Vec3::new(1155., HEIGHT / 2., Z_BOB),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(FreezeButton)
        .insert(ShopUi);

    commands
        .spawn_bundle(SpriteBundle {
            material: handles.upgrade_button.clone(),
            transform: Transform {
                translation: Vec3::new(1155., HEIGHT / 2. - 140., Z_BOB),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(UpgradeButton)
        .insert(ShopUi);

    commands
        .spawn_bundle(Text2dBundle {
            text: Text::with_section("",
                                     text_styles.love_bug_small.clone(),
                                     TextAlignment {
                                         horizontal: HorizontalAlign::Center,
                                         ..Default::default()
                                     }),
            transform: Transform {
                translation: Vec3::new(WIDTH / 2., HEIGHT - 50., 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ButtonText)
        .insert(ShopUi);
}

fn draw_effect(material: Handle<ColorMaterial>, slot: ShopSlot) -> SpriteBundle {
    SpriteBundle {
        material,
        visible: Visible {
            is_visible: false,
            is_transparent: true,
        },
        transform: Transform {
            translation: vec3(slot.x() - CARD_WIDTH / 2., slot.y() + CARD_HEIGHT / 2., Z_ABILITY),
            ..Default::default()
        },
        ..Default::default()
    }
}

struct AbilitiesStack {
    stack: Vec<(Abilities, u32)>,
    next_tick_after: f64,
}

const ABILITY_DISPLAY_TIME: f64 = 1.5;

fn played_trigger(
    time: Res<Time>,
    mut commands: Commands,
    mut ev_played: EventReader<PlayedTrigger>,
    mut ev_stats: EventWriter<StatsChanged>,
    mut ev_new_card: EventWriter<NewCard>,
    handles: Res<TextureAssets>,
    mut global_data: ResMut<GlobalData>,
    mut cards: QuerySet<(
        Query<(&mut Card, &ShopSlot)>,
        Query<(Entity, &mut Card, &ShopSlot)>
    )>,
    audio: Res<Audio>,
    music: Res<AudioAssets>,
) {
    for trigger in ev_played.iter() {
        audio.play_in_channel(music.ability_triggered.clone(), &AudioChannel::new("SFX2".to_owned()));
        commands
            .entity(trigger.0)
            .with_children(|parent| {
                parent
                    .spawn_bundle(SpriteBundle {
                        material: handles.heart.clone(),
                        transform: Transform {
                            translation: vec3(-CARD_WIDTH / 2. / CARD_SCALE, CARD_HEIGHT / 2. / CARD_SCALE, Z_ABILITY),
                            scale: vec3(1. / CARD_SCALE, 1. / CARD_SCALE, 1.),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(RemoveAfter(time.seconds_since_startup() + ABILITY_DISPLAY_TIME));
            });

        let (mut card, slot) = cards.q0_mut().get_mut(trigger.0).unwrap();
        let ability = card.base_card.ability();

        match ability {
            Abilities::SweetScent => {
                for (e, mut card, other_slot) in cards.q1_mut().iter_mut() {
                    if other_slot.row != ShopSlots::BOARD || e == trigger.0 || card.base_card.family() != Families::Mushrooms { continue; }
                    card.atk += 1;
                    card.hp += 1;
                    ev_stats.send(StatsChanged(e));
                }
            }
            Abilities::Cooperation => {
                let mut other_spiders = 0;
                for (e, mut card, other_slot) in cards.q1_mut().iter_mut() {
                    if other_slot.row != ShopSlots::BOARD || e == trigger.0 || card.base_card.family() != Families::Spiders { continue; }
                    other_spiders += 1;
                }
                let (mut card, _) = cards.q0_mut().get_mut(trigger.0).unwrap();
                card.hp += other_spiders;
                ev_stats.send(StatsChanged(trigger.0));
            }
            Abilities::Replication => {
                let mut occupied_slots = HashSet::new();
                for (_, _, other_slot) in cards.q1_mut().iter_mut() {
                    if other_slot.row != ShopSlots::BOARD { continue; }
                    occupied_slots.insert(other_slot.id);
                }
                for i in 0..=6 {
                    if !occupied_slots.contains(&i) {
                        let nanobot = add_card(Card::new(BaseCards::ROB_1, global_data.next_card_id),
                                 ShopSlot { row: ShopSlots::BOARD, id: i as u8 },
                                 &mut commands, &handles, &mut ev_new_card);
                        commands
                            .entity(nanobot)
                            .insert(Draggable { size: vec2(CARD_WIDTH / 2., CARD_HEIGHT / 2.) });
                        global_data.next_card_id += 1;
                        break;
                    }
                }
            }
            Abilities::Scanner => {
                let mut other_robots = 0;
                for (e, mut card, other_slot) in cards.q1_mut().iter_mut() {
                    if other_slot.row != ShopSlots::BOARD || e == trigger.0 || card.base_card.family() != Families::Robots { continue; }
                    other_robots += 1;
                }
                let (mut card, _) = cards.q0_mut().get_mut(trigger.0).unwrap();
                card.hp += other_robots;
                card.atk += other_robots;
                ev_stats.send(StatsChanged(trigger.0));
            }
            _ => {}
        }
    }
}

fn sold_trigger(
    time: Res<Time>,
    mut commands: Commands,
    mut ev_sold: EventReader<SoldTrigger>,
    mut ev_stats: EventWriter<StatsChanged>,
    mut ev_new_card: EventWriter<NewCard>,
    handles: Res<TextureAssets>,
    mut global_data: ResMut<GlobalData>,
    mut cards: Query<(Entity, &mut Card, &ShopSlot)>,
    bob: Query<Entity, With<Bob>>,
    audio: Res<Audio>,
    music: Res<AudioAssets>,
) {
    for trigger in ev_sold.iter() {
        let ability = trigger.0.base_card.ability();
        let triggered = trigger.0.base_card.trigger() == Triggers::Sold;

        if triggered {
            audio.play_in_channel(music.ability_triggered.clone(), &AudioChannel::new("SFX2".to_owned()));
            commands
                .entity(bob.single().unwrap())
                .with_children(|parent| {
                    parent
                        .spawn_bundle(SpriteBundle {
                            material: handles.heart.clone(),
                            transform: Transform {
                                translation: vec3(-CARD_WIDTH / 2. / CARD_SCALE, CARD_HEIGHT / 2. / CARD_SCALE, Z_ABILITY),
                                scale: vec3(1. / CARD_SCALE, 1. / CARD_SCALE, 1.),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                        .insert(RemoveAfter(time.seconds_since_startup() + ABILITY_DISPLAY_TIME));
                });
        }

        match ability {
            Abilities::Sporocarp => {
                let mut occupied_slots = HashSet::new();
                for (_, _, other_slot) in cards.iter_mut() {
                    if other_slot.row != ShopSlots::HAND { continue; }
                    occupied_slots.insert(other_slot.id);
                }
                for i in 0..=4 {
                    if !occupied_slots.contains(&i) {
                        let random_mush = BaseCards::random_mush(&mut global_data.rng);
                        let mush = add_card(Card::new(random_mush, global_data.next_card_id),
                                                   ShopSlot { row: ShopSlots::HAND, id: i as u8 },
                                                   &mut commands, &handles, &mut ev_new_card);
                        commands
                            .entity(mush)
                            .insert(Draggable { size: vec2(CARD_WIDTH / 2., CARD_HEIGHT / 2.) });
                        global_data.next_card_id += 1;
                        break;
                    }
                }
            }
            Abilities::Altruism => {
                let mut board_entities = vec![];
                for (e, _, other_slot) in cards.iter_mut() {
                    if other_slot.row != ShopSlots::BOARD { continue; }
                    board_entities.push(e);
                }
                let mut hp_ups = vec![];
                for i in 0..trigger.0.hp {
                    hp_ups.push(board_entities.choose(&mut global_data.rng));
                }
                for (e, mut card, other_slot) in cards.iter_mut() {
                    if other_slot.row != ShopSlots::BOARD { continue; }
                    let mut stats_changed = false;

                    for e2 in hp_ups.iter() {
                        if let Some(e2) = e2 {
                            if e == **e2 {
                                stats_changed = true;
                                card.hp += 1;
                            }
                        }
                    }

                    if stats_changed { ev_stats.send(StatsChanged(e)); }
                }
            }
            _ => {}
        }
    }
}

fn display_ability_animation(
    time: Res<Time>,
    mut stack_query: Query<(Entity, &mut AbilitiesStack)>,
    mut card_query: Query<(Entity, &ShopSlot, &mut Card)>,
    mut commands: Commands,
    handles: Res<TextureAssets>,
    mut player_query: Query<&mut PlayerData, With<MySelf>>,
    mut global_data: ResMut<GlobalData>,
    mut ev_new_card: EventWriter<NewCard>,
    mut ev_stats: EventWriter<StatsChanged>,
    mut ev_gold_event: EventWriter<CoinsDiff>,
    audio: Res<Audio>,
    music: Res<AudioAssets>,
) {
    if let Ok((entity_stack, mut ab_stack)) = stack_query.single_mut() {
        let t = time.seconds_since_startup();
        if ab_stack.next_tick_after < t {
            if let Some((ability, card_id)) = ab_stack.stack.pop() {
                let mut slot = None;
                for (_, &s, mut c) in card_query.iter_mut() {
                    if c.id == card_id {
                        slot = Some(s);
                    }
                }

                if let Some(slot) = slot {
                    let start = t;
                    let end = start + ABILITY_DISPLAY_TIME;
                    let material = handles.heart.clone();

                    commands
                        .spawn_bundle(draw_effect(material, slot))
                        .insert(DisplayBetweenAnimation { start, end })
                        .insert(RemoveAfter(end + 0.1));

                    ab_stack.next_tick_after = end;
                    audio.play_in_channel(music.ability_triggered.clone(), &AudioChannel::new("SFX2".to_owned()));

                    match ability {
                        Abilities::Spawn => {
                            let mut player_data = player_query.single_mut().expect("There should be a main player.");
                            if player_data.board.len() < 7 {
                                let base_card = if global_data.rng.gen() { BaseCards::SPID_1 } else { BaseCards::SPID_2 };
                                let new_card = Card::new(base_card, global_data.next_card_id);
                                global_data.next_card_id += 1;
                                player_data.board.push(new_card);
                                let new_slot = ShopSlot { row: ShopSlots::BOARD, id: player_data.board.len() as u8 - 1 };
                                add_card(new_card, new_slot, &mut commands, &handles, &mut ev_new_card);
                            }
                        }
                        Abilities::Cannibalism => {
                            if let Some((eaten_entity, _, eaten_card)) = card_query.iter_mut()
                                .filter(|(_, &s, card)|
                                    card.base_card.family() == Families::Spiders
                                        && card.id != card_id
                                        && s.row == ShopSlots::BOARD)
                                .min_by_key(|(_, _, card)| card.base_card.rank()) {
                                commands.entity(eaten_entity)
                                    .despawn_recursive();
                                let change_hp = eaten_card.hp;
                                let change_atk = eaten_card.atk;
                                for (e, _, mut c) in card_query.iter_mut() {
                                    if c.id == card_id {
                                        c.hp += change_hp;
                                        c.atk += change_atk;
                                        ev_stats.send(StatsChanged(e));
                                        break;
                                    }
                                }
                            }
                        }
                        Abilities::Upgrade => {
                            let index = slot.id;
                            for (e, &s, mut card) in card_query.iter_mut() {
                                if s.row == ShopSlots::BOARD
                                    && (s.id == index + 1 || s.id + 1 == index) {
                                    card.hp += 1;
                                    card.atk += 1;
                                    ev_stats.send(StatsChanged(e));
                                }
                            }
                        }
                        Abilities::Upload => {
                            for (e, &s, mut card) in card_query.iter_mut() {
                                if card.id == card_id {
                                    if card.hp < 3 {
                                        commands.entity(e)
                                            .despawn_recursive();
                                    } else if s.row == ShopSlots::BOARD {
                                        card.hp -= 2;
                                        ev_stats.send(StatsChanged(e))
                                    }
                                } else if card.base_card.family() == Families::Robots {
                                    card.hp += 2;
                                    ev_stats.send(StatsChanged(e));
                                }
                            }
                        }
                        Abilities::Download => {
                            let mut change_hp = 0u16;
                            let mut change_atk = 0u16;
                            for (e, &s, mut card) in card_query.iter_mut() {
                                if card.base_card.family() == Families::Robots && s.row == ShopSlots::BOARD && card.id != card_id {
                                    change_hp += 1;
                                    let atk = min(1, card.atk);
                                    change_atk += atk;
                                    card.atk -= atk;
                                    card.hp -= 1;
                                    if card.hp < 1 {
                                        commands.entity(e).despawn_recursive()
                                    } else {
                                        ev_stats.send(StatsChanged(e));
                                    }
                                }
                            }
                            for (e, _, mut card) in card_query.iter_mut() {
                                if card.id == card_id {
                                    card.hp += change_hp;
                                    card.atk += change_atk;
                                    ev_stats.send(StatsChanged(e));
                                }
                            }
                        }
                        Abilities::Slimy => {
                            for (e, _, mut card) in card_query.iter_mut() {
                                if card.id == card_id {
                                    card.hp += 1;
                                    ev_stats.send(StatsChanged(e));
                                }
                            }
                        }
                        Abilities::Roots => {
                            let mut num = 0u16;
                            for (_, &s, mut card) in card_query.iter_mut() {
                                if card.base_card.family() == Families::Mushrooms && s.row == ShopSlots::BOARD && card.id != card_id {
                                    num += 1;
                                }
                            }
                            for (e, _, mut card) in card_query.iter_mut() {
                                if card.id == card_id {
                                    card.hp += num as u16;
                                    ev_stats.send(StatsChanged(e));
                                }
                            }
                        }
                        Abilities::GoldMine => {
                            ev_gold_event.send(CoinsDiff(-1, true));
                        }
                        _ => {}
                    }
                }
            } else {
                commands.spawn().insert(StartDraggableAt(t + 0.5));
                commands.entity(entity_stack).despawn_recursive();
            }
        }
    }
}

fn add_card(card: Card, slot: ShopSlot, commands: &mut Commands, handles: &Res<TextureAssets>, ev_new_card: &mut EventWriter<NewCard>) -> Entity {
    let id = commands
        .spawn_bundle(SpriteBundle {
            material: card.base_card.handle(&handles),
            transform: card_transform(slot.x(), slot.y()),
            ..Default::default()
        })
        .insert(card)
        .insert(slot)
        .id();
    ev_new_card.send(NewCard(id, card.clone()));
    return id;
}

fn update_ui(
    time: Res<Time>,
    shop_values: Res<ShopValues>,
    coin_limit: Res<CoinLimit>,
    mut state: ResMut<State<AppState>>,
    mut texts: QuerySet<(
        Query<&mut Text, With<Coins>>,
        Query<&mut Text, With<Level>>,
        Query<(&mut Text, &BeganShop)>,
    )>,
    mut players: QuerySet<(
        Query<&PlayerData, With<MySelf>>,
        Query<&mut PlayerData, With<MyFoe>>,
    )>,
    mut global_data: ResMut<GlobalData>,
) {
    let data = players.q0().single().expect("No data for the player");
    let coins = data.coins;
    let level = data.shop_level;

    let mut coins_text = texts.q0_mut().single_mut().expect("Coins text not found.");
    coins_text.sections[0].value = format!("COINS: {}/{}", coins, coin_limit.0);

    let mut level_text = texts.q1_mut().single_mut().expect("Level text not found.");
    level_text.sections[1].value = format!("SHOP LEVEL {}", level);

    if let Ok((mut time_text, BeganShop(t0))) = texts.q2_mut().single_mut()
    {
        let remaining_time = shop_values.timer - time.seconds_since_startup() + *t0;
        time_text.sections[0].value = format!("REMAINING TIME {}s", remaining_time as u8);

        if remaining_time < 0. {
            let mut foe = players.q1_mut().single_mut().unwrap();
            foe.board = foe.ia.hand(&mut global_data);
            state.set(AppState::Fight);
        }
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
        ShopSlots::SHOP => hovered_slot.row == ShopSlots::HAND || hovered_slot.row == ShopSlots::BOARD,
        ShopSlots::BOARD => hovered_slot.row == ShopSlots::BOARD || hovered_slot.row == ShopSlots::SELL,
        ShopSlots::HAND => hovered_slot.row == ShopSlots::BOARD || hovered_slot.row == ShopSlots::HAND || hovered_slot.row == ShopSlots::SELL,
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
    mut ev_played: EventWriter<PlayedTrigger>,
    shop_values: Res<ShopValues>,
    time: Res<Time>,
    mut cards: QuerySet<(
        Query<(Entity, &ShopSlot), With<SlotHovered>>,
        Query<(Entity, &Transform, &mut ShopSlot), With<Card>>,
    )>,
    player_data: Query<(&PlayerData), With<MySelf>>,
    card: Query<&Card>,
    audio: Res<Audio>,
    music: Res<AudioAssets>,
) {
    for dropped in ev_dropped.iter() {
        // Get hovered slot and remove SlotHovered component
        let hovered_slot: Option<ShopSlot> = match cards.q0_mut().single_mut() {
            Ok((e, slot)) => {
                commands.entity(e).remove::<SlotHovered>();
                Some(slot.clone())
            }
            Err(_) => None
        };

        match hovered_slot {
            None => {
                // Find the dragged card and send it back to its slot
                for (e, transform, mut slot) in cards.q1_mut().iter_mut() {
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
                for (e, _, slot) in cards.q1_mut().iter_mut() {
                    if dropped.0 == e {
                        origin_slot = Some(slot.clone());
                    } else if destination_slot == *slot {
                        existing_entity = Some(e);
                    }
                }

                let origin_slot = origin_slot.unwrap();
                let origin_pos = (origin_slot.x(), origin_slot.y());

                let data = player_data.single().expect("Can't find player data.");

                let legal_move: bool = match origin_slot.row {
                    ShopSlots::HAND => destination_slot.row == ShopSlots::HAND ||
                        destination_slot.row == ShopSlots::SELL ||
                        destination_slot.row == ShopSlots::BOARD && existing_entity.is_none(),
                    ShopSlots::BOARD => destination_slot.row == ShopSlots::BOARD || destination_slot.row == ShopSlots::SELL,
                    ShopSlots::SHOP => (destination_slot.row == ShopSlots::HAND || destination_slot.row == ShopSlots::BOARD) && existing_entity.is_none() && data.coins >= shop_values.buy as u16,
                    ShopSlots::SELL => false,
                };
                // println!["Move: {:?} {} -> {:?} {} : {}", &origin_slot.row, &origin_slot.id, &destination_slot.row, &destination_slot.id, legal_move];

                // Move the dragged card to its new slot (or old slot if the move isn't legal)
                for (e, transform, mut slot) in cards.q1_mut().iter_mut() {
                    if dropped.0 == e {
                        if legal_move {
                            slot.row = destination_slot.row;
                            slot.id = destination_slot.id;
                            commands
                                .entity(e)
                                .insert(animate_fast(&time, (transform.translation.x, transform.translation.y), (destination_slot.x(), destination_slot.y())));
                            if destination_slot.row == ShopSlots::SELL {
                                commands.entity(e).insert(Sold);
                            } else if origin_slot.row == ShopSlots::SHOP {
                                ev_coins.send(CoinsDiff(shop_values.buy, false));
                                audio.play_in_channel(music.buy_card.clone(), &AudioChannel::new("SFX".to_owned()));
                            } else {
                                audio.play_in_channel(music.place_card.clone(), &AudioChannel::new("SFX".to_owned()));
                            }
                            let card = card.get(e).unwrap();
                            if card.base_card.trigger() == Triggers::Played && origin_slot.row != ShopSlots::BOARD && destination_slot.row == ShopSlots::BOARD {
                                ev_played.send(PlayedTrigger(e));
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
                        for (e, transform, mut slot) in cards.q1_mut().iter_mut() {
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
    shop_values: Res<ShopValues>,
    mut ev_transition: EventReader<TransitionOver>,
    mut ev_coins: EventWriter<CoinsDiff>,
    mut ev_sold: EventWriter<SoldTrigger>,
    mut cards: Query<(Entity, &ShopSlot, &Card), With<Card>>,
    audio: Res<Audio>,
    music: Res<AudioAssets>,
) {
    for transition in ev_transition.iter() {
        for (e, slot, card) in cards.iter_mut() {
            if e == transition.0 && slot.row == ShopSlots::SELL {
                audio.play_in_channel(music.sell_card.clone(), &AudioChannel::new("SFX".to_owned()));
                ev_sold.send(SoldTrigger(card.clone()));
                commands.entity(transition.0).despawn_recursive();
                ev_coins.send(CoinsDiff(shop_values.sell, false));
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
        if !diff.1 && diff.0 < 0 && player_data.coins + (-diff.0) as u16 > coin_limit.0 {
            player_data.coins = max(coin_limit.0, player_data.coins);
            break;
        }
        player_data.coins = (player_data.coins as i16 - diff.0 as i16) as u16;
    }
}

fn on_exit(
    mut commands: Commands,
    cards: Query<(Entity, &Card, &ShopSlot)>,
    mut player_data: Query<&mut PlayerData, With<MySelf>>,
) {
    let mut player_data = player_data.single_mut().expect("There should only be one player tagged with myself");
    let mut new_board: Vec<(u8, Card)> = vec![];
    let mut new_hand: Vec<(u8, Card)> = vec![];
    for (e, &card, slot) in cards.iter() {
        match slot.row {
            ShopSlots::BOARD => {
                new_board.push((slot.id, card));
            }
            ShopSlots::HAND => {
                new_hand.push((slot.id, card));
            }
            ShopSlots::SHOP => {}
            ShopSlots::SELL => {}
        };
        commands.entity(e).despawn_recursive();
    }
    new_board.sort_by_key(|t| t.0);
    new_hand.sort_by_key(|t| t.0);
    player_data.board = new_board.iter().map(|t| t.1).collect();
    player_data.hand = new_hand.iter().map(|t| t.1).collect();
}

struct StartDraggableAt(f64);

fn start_draggable(
    start_draggable_query: Query<(Entity, &StartDraggableAt)>,
    card_query: Query<Entity, With<Card>>,
    time: Res<Time>,
    text_styles: Res<TextStyles>,
    mut commands: Commands,
    mut can_refresh: ResMut<CanRefresh>,
) {
    for (es, &StartDraggableAt(t)) in start_draggable_query.iter() {
        if time.seconds_since_startup() > t {
            commands.entity(es).despawn_recursive();

            for e in card_query.iter() {
                commands.entity(e)
                    .insert(Draggable {
                        size: vec2(CARD_WIDTH / 2., CARD_HEIGHT / 2.),
                    });
            }

            commands.spawn_bundle(
                text_bundle_at_corner(
                    Corners::TopRight,
                    vec!["REMAINING TIME 60s".to_string()],
                    &text_styles.love_bug_small,
                )).insert(BeganShop(time.seconds_since_startup()));

            can_refresh.0 = true;
        }
    }
}

fn handle_buttons(
    mut player_data: Query<&mut PlayerData, With<MySelf>>,
    btn: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    shop_values: Res<ShopValues>,
    queries: QuerySet<(
        Query<&Transform, With<MainCamera>>,
        Query<&Transform, With<RefreshButton>>,
        Query<&Transform, With<FreezeButton>>,
        Query<&Transform, With<UpgradeButton>>,
    )>,
    card_query: Query<(Entity, &Card, &ShopSlot)>,
    mut button_text: Query<&mut Text, With<ButtonText>>,
    mut frozen_shop: ResMut<ShopFrozen>,
    mut commands: Commands,
    mut global_data: ResMut<GlobalData>,
    handles: Res<TextureAssets>,
    mut ev_new_card: EventWriter<NewCard>,
    audio: Res<Audio>,
    music: Res<AudioAssets>,
) {
    let window = windows.get_primary().unwrap();
    if let Some(cursor) = cursor_pos(window, queries.q0().single().unwrap()) {
        let mut player_data = player_data.single_mut().unwrap();

        let transform = queries.q1().single().unwrap();
        if overlap(cursor.xyz(), transform.translation, (50., 50.)) {
            button_text.single_mut().unwrap().sections[0].value = format!("Refresh cards for {} coins.", shop_values.refresh);
            if btn.just_pressed(MouseButton::Left) && player_data.coins >= shop_values.refresh {
                audio.play_in_channel(music.refresh.clone(), &AudioChannel::new("SFX".to_owned()));
                player_data.coins -= shop_values.refresh;
                *frozen_shop = ShopFrozen(None);
                for (e, &card, &slot) in card_query.iter() {
                    if slot.row == ShopSlots::SHOP {
                        commands.entity(e).despawn_recursive();
                    }
                }
                for (i, &base_card) in ShopManager::shop_inventory(player_data.shop_level, &mut global_data.rng).iter().enumerate() {
                    let id = global_data.next_card_id;
                    global_data.next_card_id += 1;
                    let card_id = add_card(Card::new(base_card, id),
                             ShopSlot { row: ShopSlots::SHOP, id: i as u8 },
                             &mut commands, &handles, &mut ev_new_card);
                    commands
                        .entity(card_id)
                        .insert(Draggable { size: vec2(CARD_WIDTH / 2., CARD_HEIGHT / 2.) });
                }
            }
            return;
        }

        let transform = queries.q2().single().unwrap();
        if overlap(cursor.xyz(), transform.translation, (50., 50.)) {
            button_text.single_mut().unwrap().sections[0].value =
                if frozen_shop.0.is_none() {
                    format!("Freeze cards for {} coins.", shop_values.freeze)
                } else {
                    "Shop already frozen.".to_string()
                };
            if btn.just_pressed(MouseButton::Left) && player_data.coins >= shop_values.freeze && frozen_shop.0.is_none() {
                audio.play_in_channel(music.freeze.clone(), &AudioChannel::new("SFX".to_owned()));
                player_data.coins -= shop_values.freeze;
                frozen_shop.0 = Some(
                    card_query.iter()
                        .filter_map(|(_, &card, &slot)|
                            if slot.row == ShopSlots::SHOP {
                                Some((slot.id, card))
                            } else { None })
                        .collect()
                );
            };
            return;
        }

        let transform = queries.q3().single().unwrap();
        if overlap(cursor.xyz(), transform.translation, (50., 50.)) {
            let upgrade_cost: i16 = match player_data.shop_level {
                1 => 4,
                2 => 6,
                3 => 8,
                _ => -1,
            };
            if upgrade_cost == -1 {
                button_text.single_mut().unwrap().sections[0].value = "The shop can't be upgraded anymore.".to_string();
                return;
            } else {
                button_text.single_mut().unwrap().sections[0].value = format!("Upgrade the shop for {} coins.", upgrade_cost);
                if btn.just_pressed(MouseButton::Left) && player_data.coins >= upgrade_cost as u16 {
                    audio.play_in_channel(music.level_up.clone(), &AudioChannel::new("SFX".to_owned()));
                    player_data.coins -= upgrade_cost as u16;
                    player_data.shop_level += 1;
                }
                return;
            }
        }
    }
    button_text.single_mut().unwrap().sections[0].value = "".to_string();
}