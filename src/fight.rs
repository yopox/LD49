use bevy::math::vec3;
use bevy::prelude::*;
use derive_more::Display;

use crate::{AppState, GlobalData, Handles, HEIGHT, MySelf, PlayerData, WIDTH};
use crate::abs::{CombatEvents, simulate_combat};
use crate::card::{Abilities, Card, CARD_HEIGHT, NewCard, StatsChanged};
use crate::font::TextStyles;
use crate::ui::{easing, StateBackground, TranslationAnimation};
use crate::util::{card_transform, cleanup_system, Coins, Corners, Level, PlayerHP, text_bundle_at_corner, Z_BACKGROUND};

pub struct FightPlugin;

impl Plugin for FightPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_event::<Translation>()
            .add_event::<RemoveCard>()
            .add_event::<StatsChange>()
            .add_event::<ApplyEffect>()
            .add_event::<PlayersAttack>()
            .add_event::<GoldChange>()
            .add_system_set(
                SystemSet::on_enter(AppState::Fight)
                    .with_system(setup_fight.system().label("setup_fight"))
                    .with_system(draw_fight.system())
            )
            .add_system_set(
                SystemSet::on_update(AppState::Fight)
                    .with_system(event_dispatcher.system().label("event_dispatcher"))
                    .with_system(update_ui.system().label("update_ui"))
            )
            .add_system_set(
                SystemSet::on_update(AppState::Fight).after("event_dispatcher")
                    .with_system(translation_animation_producer.system())
                    .with_system(stat_change_producer.system())
                    .with_system(remove_card_producer.system())
                    .with_system(apply_effect_producer.system())
                    .with_system(players_attack_producer.system())
                    .with_system(gold_change_producer.system())
            )
            .add_system_set(
                SystemSet::on_exit(AppState::Fight)
                    .with_system(on_exit.system().label("on-exit"))
            )
            .add_system_set(
                SystemSet::on_exit(AppState::Fight)
                    .after("on-exit").label("cleanup")
                    .with_system(cleanup_system::<FightSlot>.system())
                    .with_system(cleanup_system::<FightEventsStack>.system())
                    .with_system(cleanup_system::<StateBackground>.system())
                    .with_system(cleanup_system::<ExtraCoins>.system())
                    .with_system(cleanup_system::<Level>.system())
                    .with_system(cleanup_system::<MyHP>.system())
                    .with_system(cleanup_system::<FoeHP>.system())
                    .with_system(cleanup_system::<FightBackup>.system())
            )
        ;
    }
}

struct WaitUntil(f64);

struct ExtraCoins;

struct MyHP;

struct FoeHP;

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum FightPlayers {
    MySelf,
    MyFoe,
}

#[derive(PartialEq, Eq, Clone, Copy, Display)]
pub enum FightSlotHeight {
    MySelf,
    MyFoe,
    FightingMySelf,
    FightingMyFoe,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct FightSlot {
    who: FightSlotHeight,
    index: u8,
}

impl FightSlot {
    fn x(&self) -> f32 {
        256. + 128. * self.index as f32
    }

    fn y(&self) -> f32 {
        match self.who {
            FightSlotHeight::MySelf => HEIGHT - 576.,
            FightSlotHeight::MyFoe => HEIGHT - 160.,
            FightSlotHeight::FightingMySelf => HEIGHT - 576. + CARD_HEIGHT + 16.,
            FightSlotHeight::FightingMyFoe => HEIGHT - 160. - CARD_HEIGHT - 16.,
        }
    }
}

pub struct MyFoe;

pub struct FightBackup {
    who: FightPlayers,
}

pub struct FightEventsStack {
    stack: Vec<FightEvents>,
}

fn add_card(card: Card, slot: FightSlot, commands: &mut Commands, handles: &Res<Handles>, ev_new_card: &mut EventWriter<NewCard>) {
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
}

fn setup_fight(
    mut commands: Commands,
    handles: Res<Handles>,
    time: Res<Time>,
    mut global_data: ResMut<GlobalData>,
    mut ev_new_card: EventWriter<NewCard>,
    queries: QuerySet<(
        Query<(Entity, &PlayerData), With<MySelf>>,
        Query<(Entity, &PlayerData), With<MyFoe>>,
    )>,
) {
    let (e_myself, myself) = queries.q0().single().expect("There should be only one player tagged MySelf");
    let mut myself_cloned = myself.clone();
    let myself_cloned_again = myself.clone();

    let (e_my_foe, my_foe) = queries.q1().single().expect("There should be only one player tagged MyFoe");
    let mut my_foe_cloned = my_foe.clone();
    let my_foe_cloned_again = my_foe.clone();

    let my_id = myself_cloned.id;

    let mut index = 0u8;
    for &card in &myself_cloned.board {
        add_card(card, FightSlot { who: FightSlotHeight::MySelf, index },
                 &mut commands, &handles, &mut ev_new_card);
        index += 1;
    }

    let mut index = 0u8;
    for &card in &my_foe_cloned.board {
        add_card(card, FightSlot { who: FightSlotHeight::MyFoe, index },
                 &mut commands, &handles, &mut ev_new_card);
        index += 1;
    }

    let events = simulate_combat(myself_cloned_again, my_foe_cloned_again, &mut global_data.rng);

    let mut stack = Vec::with_capacity(events.len());
    for e in events {
        match e {
            CombatEvents::Attack { att_id, att_card_index, def_card_index, change_def_hp } => {
                let att = if att_id == my_id { FightPlayers::MySelf } else { FightPlayers::MyFoe };
                let def = other_player(att);
                let att_base = FightSlot { who: to_base_height(att), index: att_card_index };
                let att_post = FightSlot { who: to_fighting_height(att), index: def_card_index };
                let def_base = FightSlot { who: to_base_height(def), index: def_card_index };
                // Translation to fight
                stack.push(FightEvents::Translation(Translation { from: att_base, to: att_post }));
                // StatChange
                stack.push(FightEvents::StatsChange(StatsChange { slot: def_base, at: 0, hp: change_def_hp }));
                // Translation back to base
                stack.push(FightEvents::Translation(Translation { from: att_post, to: att_base }));
            }
            CombatEvents::Death { player_id, card_index } => {
                let player = if player_id == my_id { FightPlayers::MySelf } else { FightPlayers::MyFoe };
                let slot = FightSlot { who: to_base_height(player), index: card_index };
                stack.push(FightEvents::RemoveCard(RemoveCard(slot)));
            }
            CombatEvents::StatsChange { player_id, card_index, hp, at } => {
                let player = if player_id == my_id { FightPlayers::MySelf } else { FightPlayers::MyFoe };
                let slot = FightSlot { who: to_base_height(player), index: card_index };
                stack.push(FightEvents::StatsChange(StatsChange { slot, at, hp }));
            }
            CombatEvents::ApplyAbility { card_index, player_id, ability, card_id } => {
                let player = if player_id == my_id { FightPlayers::MySelf } else { FightPlayers::MyFoe };
                let slot = FightSlot { who: to_base_height(player), index: card_index };
                stack.push(FightEvents::ApplyEffect(ApplyEffect(slot)));

                match ability {
                    Abilities::Gigantism => {
                        if player_id == my_id {
                            for mut card in myself_cloned.board.iter_mut() {
                                if card.id == card_id {
                                    card.atk += 1;
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            CombatEvents::GoldChange { player_id, change } => {
                let who = if player_id == my_id {
                    myself_cloned.extra_coins = (myself_cloned.extra_coins as i32 + change) as u16;
                    FightPlayers::MySelf
                } else {
                    my_foe_cloned.extra_coins = (my_foe_cloned.extra_coins as i32 + change) as u16;
                    FightPlayers::MyFoe
                };
                stack.push(FightEvents::GoldChange(GoldChange { who, change }));
            }
            CombatEvents::PlayersAttack { att_id, change_def_hp } => {
                let on = if att_id == my_id {
                    my_foe_cloned.hp = (my_foe_cloned.hp as i32 + change_def_hp) as u16;
                    FightPlayers::MyFoe
                } else {
                    myself_cloned.hp = (myself_cloned.hp as i32 + change_def_hp) as u16;
                    FightPlayers::MySelf
                };
                stack.push(FightEvents::PlayersAttack(PlayersAttack { on, change: change_def_hp }))
            }
        }
    }

    stack.reverse();

    commands.spawn()
        .insert(myself_cloned)
        .insert(FightBackup { who: FightPlayers::MySelf });
    commands.spawn()
        .insert(my_foe_cloned)
        .insert(FightBackup { who: FightPlayers::MyFoe });

    commands.spawn().insert(FightEventsStack { stack });
    commands.spawn().insert(WaitUntil(time.seconds_since_startup()));
}

fn draw_fight(
    mut commands: Commands,
    handles: Res<Handles>,
    text_styles: Res<TextStyles>,
    global_data: Res<GlobalData>,
) {
    commands.spawn_bundle(SpriteBundle {
        material: handles.fight_bg.clone(),
        transform: Transform {
            translation: Vec3::new(WIDTH / 2., HEIGHT / 2., Z_BACKGROUND),
            ..Default::default()
        },
        ..Default::default()
    }).insert(StateBackground);


    commands
        .spawn_bundle(text_bundle_at_corner(
            Corners::TopLeft,
            vec![format!("TURN {}\n", global_data.turn)],
            &text_styles.love_bug_small,
        ))
        .insert(Level);

    commands
        .spawn_bundle(text_bundle_at_corner(
            Corners::BottomLeft,
            vec!["EXTRA COINS: 0".to_string()],
            &text_styles.love_bug_small,
        ))
        .insert(ExtraCoins);

    commands.spawn_bundle(
        text_bundle_at_corner(
            Corners::BottomRight,
            vec![format!("YOUR HP: 0")],
            &text_styles.love_bug_small,
        )
    ).insert(MyHP);

    commands.spawn_bundle(
        text_bundle_at_corner(
            Corners::TopRight,
            vec![format!("YOUR FOE HP: 0")],
            &text_styles.love_bug_small,
        )
    ).insert(FoeHP);
}

fn to_base_height(p: FightPlayers) -> FightSlotHeight {
    match p {
        FightPlayers::MySelf => FightSlotHeight::MySelf,
        FightPlayers::MyFoe => FightSlotHeight::MyFoe,
    }
}

fn to_fighting_height(p: FightPlayers) -> FightSlotHeight {
    match p {
        FightPlayers::MySelf => FightSlotHeight::FightingMyFoe,
        FightPlayers::MyFoe => FightSlotHeight::FightingMySelf,
    }
}

fn other_player(p: FightPlayers) -> FightPlayers {
    match p {
        FightPlayers::MySelf => FightPlayers::MyFoe,
        FightPlayers::MyFoe => FightPlayers::MySelf,
    }
}

struct Translation {
    from: FightSlot,
    to: FightSlot,
}

struct RemoveCard(FightSlot);

struct StatsChange {
    slot: FightSlot,
    hp: i32,
    at: i32,
}

struct ApplyEffect(FightSlot);

struct GoldChange {
    who: FightPlayers,
    change: i32,
}

struct PlayersAttack {
    on: FightPlayers,
    change: i32,
}

enum FightEvents {
    Translation(Translation),
    RemoveCard(RemoveCard),
    StatsChange(StatsChange),
    ApplyEffect(ApplyEffect),
    PlayersAttack(PlayersAttack),
    GoldChange(GoldChange),
}

fn event_dispatcher(
    time: Res<Time>,
    mut queries: QuerySet<(
        Query<&mut FightEventsStack>,
        Query<(Entity, &WaitUntil)>
    )>,
    mut commands: Commands,
    mut ew_translation: EventWriter<Translation>,
    mut ew_remove_card: EventWriter<RemoveCard>,
    mut ew_stats_change: EventWriter<StatsChange>,
    mut ew_apply_effect: EventWriter<ApplyEffect>,
    mut ew_players_attack: EventWriter<PlayersAttack>,
    mut ew_gold_change: EventWriter<GoldChange>,
    mut app_state: ResMut<State<AppState>>,
) {
    let mut should_dispatch = false;
    for (e, WaitUntil(t0)) in queries.q1().iter() {
        if time.seconds_since_startup() > *t0 {
            commands.entity(e).despawn_recursive();
            should_dispatch = true;
        }
    }
    if should_dispatch {
        let mut stack = queries.q0_mut().single_mut().expect("There should only be one stack");
        if let Some(e) = stack.stack.pop() {
            match e {
                FightEvents::Translation(t) => {
                    ew_translation.send(t);
                }
                FightEvents::RemoveCard(r) => {
                    ew_remove_card.send(r);
                }
                FightEvents::StatsChange(s) => {
                    ew_stats_change.send(s);
                }
                FightEvents::ApplyEffect(a) => {
                    ew_apply_effect.send(a);
                }
                FightEvents::PlayersAttack(pa) => {
                    ew_players_attack.send(pa);
                }
                FightEvents::GoldChange(g) => {
                    ew_gold_change.send(g);
                }
            }
        } else {
            app_state.set(AppState::Shop);
        }
    }
}

fn translation_animation_producer(
    mut er: EventReader<Translation>,
    mut commands: Commands,
    query: Query<(Entity, &FightSlot)>,
    time: Res<Time>,
) {
    for Translation { from, to } in er.iter() {
        for (e, mut slot) in query.iter() {
            if slot == from {
                let duration = 1.3;
                let t0 = time.seconds_since_startup();
                commands.entity(e)
                    .remove::<FightSlot>()
                    .insert(translate_slots(t0, *slot, *to, duration))
                    .insert(*to);
                commands.spawn()
                    .insert(WaitUntil(t0 + duration as f64));
            }
        }
    }
}

fn translate_slots(t0: f64, from: FightSlot, to: FightSlot, duration: f64) -> TranslationAnimation {
    let start = vec3(from.x(), from.y(), 0.);
    let end = vec3(to.x(), to.y(), 0.);
    TranslationAnimation {
        f: easing::Functions::CubicInOut,
        translation: end - start,
        start,
        duration,
        t0,
    }
}

fn stat_change_producer(
    mut er_stats_change: EventReader<StatsChange>,
    mut query: Query<(Entity, &mut Card, &FightSlot)>,
    mut commands: Commands,
    mut ev_stats: EventWriter<StatsChanged>,
    time: Res<Time>,
) {
    for event in er_stats_change.iter() {
        for (e, mut card, &slot) in query.iter_mut() {
            if slot == event.slot {
                println!("Changing stats at slot {}.{}", slot.who, slot.index);
                card.hp = (card.hp as i32 + event.hp) as u16;
                card.atk = (card.atk as i32 + event.at) as u16;
                commands.spawn().insert(WaitUntil(time.seconds_since_startup() + 0.5));
                ev_stats.send(StatsChanged(e));
            }
        }
    }
}

fn remove_card_producer(
    time: Res<Time>,
    mut er_remove_card_event: EventReader<RemoveCard>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut FightSlot)>,
) {
    let t0 = time.seconds_since_startup();
    let removed_slots: Vec<FightSlot> = er_remove_card_event.iter().map(|RemoveCard(t)| *t).collect();

    if removed_slots.is_empty() {
        return;
    }

    let mut translated = false;
    for (e, mut slot) in query.iter_mut() {
        if removed_slots.contains(&slot) {
            commands.entity(e)
                .despawn_recursive();
        } else {
            let removed_before: usize = removed_slots.iter().filter(|removed_slot| {
                slot.who == removed_slot.who && removed_slot.index < slot.index
            }).count();
            if removed_before != 0 {
                translated = true;
                let start = slot.clone();
                slot.who = start.who;
                slot.index = start.index - removed_before as u8;
                commands.entity(e)
                    .insert(translate_slots(t0, start, *slot, 1.3));
            }
        }
    }
    let wait_duration = if translated { 1.3 } else { 0.5 };
    commands.spawn().insert(WaitUntil(t0 + wait_duration));
}

fn apply_effect_producer(
    mut er: EventReader<ApplyEffect>,
    mut commands: Commands,
    time: Res<Time>,
) {
    if er.iter().count() != 0 {
        commands.spawn().insert(WaitUntil(time.seconds_since_startup() + 0.5));
        println!("Applying some effects");
    }
}

fn players_attack_producer(
    mut er: EventReader<PlayersAttack>,
    mut commands: Commands,
    time: Res<Time>,
    mut queries: QuerySet<(
        Query<&mut PlayerData, With<MySelf>>,
        Query<&mut PlayerData, With<MyFoe>>,
    )>,
) {
    for PlayersAttack { on, change } in er.iter() {
        commands.spawn().insert(WaitUntil(time.seconds_since_startup() + 0.5));

        let mut def_data =
            if *on == FightPlayers::MySelf {
                queries.q0_mut().single_mut().expect("Cannot find main player")
            } else {
                queries.q1_mut().single_mut().expect("Main player should have a foe")
            };
        def_data.hp = (def_data.hp as i32 + change) as u16;
    }
}

fn on_exit(
    mut query: QuerySet<(
        Query<&mut PlayerData, With<MySelf>>,
        Query<&mut PlayerData, With<MyFoe>>,
        Query<(&PlayerData, &FightBackup)>
    )>
) {
    let mut my_new_data = None;
    let mut foe_new_data = None;
    for (data, &FightBackup { who}) in query.q2().iter() {
        match who {
            FightPlayers::MySelf => {
                my_new_data = Some(data.clone());
            }
            FightPlayers::MyFoe => {
                foe_new_data = Some(data.clone());
            }
        }
    }

    if let Some(data) = my_new_data {
        let mut my_data = query.q0_mut().single_mut().expect("There should be one main player");
        *my_data = data;
    }
    if let Some(data) = foe_new_data {
        let mut foe_data = query.q1_mut().single_mut().expect("There should be one main player");
        *foe_data = data;
    }
}

fn gold_change_producer(
    mut er: EventReader<GoldChange>,
    mut query: QuerySet<(
        Query<&mut PlayerData, With<MySelf>>,
        Query<&mut PlayerData, With<MyFoe>>,
    )>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let mut should_trigger_wait = false;
    for GoldChange { who, change } in er.iter() {
        let mut player_data = match who {
            FightPlayers::MySelf => query.q0_mut().single_mut().expect("Cannot get main player"),
            FightPlayers::MyFoe => query.q1_mut().single_mut().expect("Cannot get opponent"),
        };
        player_data.hp = (player_data.hp as i32 + change) as u16;
        should_trigger_wait = true;
    }
    if should_trigger_wait {
        commands.spawn().insert(WaitUntil(time.seconds_since_startup() + 0.5));
    }
}

fn update_ui(
    player_queries: QuerySet<(
        Query<&PlayerData, With<MySelf>>,
        Query<&PlayerData, With<MyFoe>>
    )>,
    mut text_queries: QuerySet<(
        Query<&mut Text, With<ExtraCoins>>,
        Query<&mut Text, With<MyHP>>,
        Query<&mut Text, With<FoeHP>>
    )>,
) {
    let my_data = player_queries.q0().single().expect("No data for the player");
    let extra_coins = my_data.extra_coins;
    let my_hp = my_data.hp;

    let foe_data = player_queries.q1().single().expect("Only one foe");
    let foe_name = &foe_data.name;
    let foe_hp = foe_data.hp;

    let mut coins_text = text_queries.q0_mut().single_mut().expect("Coins text not found.");
    coins_text.sections[0].value = format!("EXTRA COINS: + {}", extra_coins);

    let mut my_hp_text = text_queries.q1_mut().single_mut().expect("Coins text not found.");
    my_hp_text.sections[0].value = format!("YOUR HP: {}", my_hp);

    let mut foe_hp_text = text_queries.q2_mut().single_mut().expect("Coins text not found.");
    foe_hp_text.sections[0].value = format!("{}'S HP: {}", foe_name, foe_hp);
}