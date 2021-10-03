use bevy::ecs::component::Component;
use bevy::math::vec3;
use bevy::prelude::*;
use derive_more::Display;

use crate::{AppState, GlobalData, Handles, HEIGHT, MySelf, PlayerData};
use crate::abs::{CombatEvents, simulate_combat};
use crate::card::{Abilities, Card, CARD_HEIGHT};
use crate::ui::{easing, TranslationAnimation};
use crate::util::card_transform;

pub struct FightPlugin;

impl Plugin for FightPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_event::<Translation>()
            .add_event::<RemoveCard>()
            .add_event::<StatsChange>()
            .add_event::<ApplyEffect>()
            .add_event::<PlayersAttack>()
            .add_system_set(
                SystemSet::on_enter(AppState::Fight)
                    .with_system(setup_fight.system().label("setup_fight"))
            )
            .add_system_set(
                SystemSet::on_update(AppState::Fight)
                    .with_system(event_dispatcher.system().label("event_dispatcher"))
            )
            .add_system_set(
                SystemSet::on_update(AppState::Fight).after("event_dispatcher")
                    .with_system(translation_animation_producer.system())
                    .with_system(stat_change_producer.system())
                    .with_system(remove_card_producer.system())
                    .with_system(apply_effect_producer.system())
                    .with_system(players_attack_producer.system())
            )
            .add_system_set(
                SystemSet::on_exit(AppState::Fight)
                    .with_system(on_exit.system().label("on-exit"))
            )
            .add_system_set(
                SystemSet::on_exit(AppState::Fight)
                    .before("on-exit").label("cleanup")
                    .with_system(cleanup_system::<MySelf>.system())
                    .with_system(cleanup_system::<MyFoe>.system())
                    .with_system(cleanup_system::<FightSlot>.system())
            )
        ;
    }
}

struct WaitUntil(f64);

#[derive(Copy, Clone)]
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

fn add_card(card: Card, slot: FightSlot, commands: &mut Commands, handles: &Res<Handles>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: card.card_type.handle(&handles),
            transform: card_transform(slot.x(), slot.y()),
            ..Default::default()
        })
        .insert(card)
        .insert(slot)
    ;
}

fn setup_fight(
    mut commands: Commands,
    handles: Res<Handles>,
    time: Res<Time>,
    mut global_data: ResMut<GlobalData>,
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
                 &mut commands, &handles);
        index += 1;
    }

    let mut index = 0u8;
    for &card in &my_foe_cloned.board {
        add_card(card, FightSlot { who: FightSlotHeight::MyFoe, index },
                 &mut commands, &handles);
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
                                    card.at += 1;
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
            CombatEvents::GoldChange { player_id, change } => {
                if player_id == my_id {
                    myself_cloned.extra_coins = (myself_cloned.extra_coins as i32 + change) as u16;
                } else {
                    my_foe_cloned.extra_coins = (my_foe_cloned.extra_coins as i32 + change) as u16;
                }
            }
            CombatEvents::PlayersAttack { att_id, change_def_hp } => {
                let who = if att_id == my_id {
                    myself_cloned.hp = (myself_cloned.hp as i32 + change_def_hp) as u16;
                    FightPlayers::MySelf
                } else {
                    my_foe_cloned.hp = (my_foe_cloned.hp as i32 + change_def_hp) as u16;
                    FightPlayers::MyFoe
                };
                stack.push(FightEvents::PlayersAttack(PlayersAttack { who, change: change_def_hp }))
            }
        }
    }

    stack.reverse();

    commands.entity(e_myself)
        .remove::<MySelf>()
        .insert(FightBackup { who: FightPlayers::MySelf });
    commands.entity(e_my_foe)
        .remove::<MyFoe>()
        .insert(FightBackup { who: FightPlayers::MyFoe });
    commands.spawn()
        .insert(myself_cloned)
        .insert(MySelf);
    commands.spawn()
        .insert(my_foe_cloned)
        .insert(MyFoe);
    commands.spawn().insert(FightEventsStack { stack });
    commands.spawn().insert(WaitUntil(time.seconds_since_startup()));
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
    who: FightPlayers,
    change: i32,
}

enum FightEvents {
    Translation(Translation),
    RemoveCard(RemoveCard),
    StatsChange(StatsChange),
    ApplyEffect(ApplyEffect),
    PlayersAttack(PlayersAttack),
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
    mut query: Query<(&mut Card, &FightSlot)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for event in er_stats_change.iter() {
        for (mut card, &slot) in query.iter_mut() {
            if slot == event.slot {
                println!("Changing stats at slot {}.{}", slot.who, slot.index);
                card.hp = (card.hp as i32 + event.hp) as u16;
                card.at = (card.at as i32 + event.at) as u16;
                commands.spawn().insert(WaitUntil(time.seconds_since_startup() + 0.5));
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
) {
    if er.iter().count() != 0 {
        commands.spawn().insert(WaitUntil(time.seconds_since_startup() + 0.5));
        println!("PlayersAttack ... ");
    }
}

fn cleanup_system<T: Component>(
    mut commands: Commands,
    q: Query<Entity, With<T>>,
) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn on_exit(
    query: Query<(Entity, &FightBackup)>,
    mut commands: Commands,
) {
    for (e, &FightBackup { who }) in query.iter() {
        match who {
            FightPlayers::MySelf => commands.entity(e).insert(MySelf),
            FightPlayers::MyFoe => commands.entity(e).insert(MyFoe),
        };
    }
}
