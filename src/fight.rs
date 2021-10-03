use bevy::math::vec2;
use bevy::prelude::*;

use crate::{AppState, Handles, HEIGHT, MySelf, PlayerData, WIDTH};
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
    queries: QuerySet<(
        Query<&PlayerData, With<MySelf>>,
        Query<&PlayerData, With<MyFoe>>
    )>,
) {
    let myself = queries.q0().single().expect("There should be only one player tagged MySelf");
    let myfoe = queries.q1().single().expect("There should be only one player tagged MyFoe");

    let mut index = 0u8;
    for &card in &myself.board {
        add_card(card, FightSlot { who: FightPlayers::MySelf, index },
                 &mut commands, &handles);
        index += 1;
    }

    let mut index = 0u8;
    for &card in &myfoe.board {
        add_card(card, FightSlot { who: FightPlayers::MyFoe, index },
                 &mut commands, &handles);
        index += 1;
    }
}
