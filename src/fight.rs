use bevy::math::vec2;
use bevy::prelude::*;
use rand::Rng;

use crate::{AppState, GlobalData, Handles, HEIGHT, MySelf, PlayerData, WIDTH};
use crate::abs::{CombatEvents, simulate_combat};
use crate::card::Card;
use crate::util::card_transform;

pub struct FightPlugin;

impl Plugin for FightPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Fight)
                .with_system(setup_fight.system())
        )
        ;
    }
}

pub enum FightPlayers {
    MySelf,
    MyFoe,
}

pub struct FightSlot {
    who: FightPlayers,
    index: u8,
}

impl FightSlot {
    fn x(&self) -> f32 {
        256. + 128. * self.index as f32
    }

    fn y(&self) -> f32 {
        match self.who {
            FightPlayers::MySelf => HEIGHT - 576.,
            FightPlayers::MyFoe => HEIGHT - 160.,
        }
    }
}

pub struct MyFoe;

pub struct FightBackup {
    who: FightPlayers,
}

pub struct FightEventsStack {
    stack: Vec<CombatEvents>,
}

fn add_card(card: Card, slot: FightSlot, commands: &mut Commands, handles: &Res<Handles>) {
    commands
        .spawn_bundle(SpriteBundle {
            material: card.card_id.handle(&handles),
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
    mut queries: QuerySet<(
        Query<(Entity, &PlayerData), With<MySelf>>,
        Query<(Entity, &PlayerData), With<MyFoe>>,
        Query<&mut GlobalData>,
    )>,
) {
    let (e_myself, myself) = queries.q0().single().expect("There should be only one player tagged MySelf");
    let (e_my_foe, my_foe) = queries.q1().single().expect("There should be only one player tagged MyFoe");

    let mut myself_cloned = myself.clone();
    let mut my_foe_cloned = my_foe.clone();
    let mut myself_cloned_again = myself.clone();
    let mut my_foe_cloned_again = my_foe.clone();

    let mut index = 0u8;
    for &card in &myself_cloned.board {
        add_card(card, FightSlot { who: FightPlayers::MySelf, index },
                 &mut commands, &handles);
        index += 1;
    }

    let mut index = 0u8;
    for &card in &my_foe_cloned.board {
        add_card(card, FightSlot { who: FightPlayers::MyFoe, index },
                 &mut commands, &handles);
        index += 1;
    }

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

    let mut global_data = queries.q2_mut().single_mut().expect("There should be only one global data.");

    let events = simulate_combat(myself_cloned_again, my_foe_cloned_again, &mut global_data.rng);

    commands.spawn()
        .insert(FightEventsStack {stack: events});
}
