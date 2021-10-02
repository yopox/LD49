use bevy::prelude::*;

use crate::{AppState, WIDTH, HEIGHT};
use crate::Handles;
use crate::card::*;

pub struct ShopPlugin;

impl Plugin for ShopPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system_set(
                SystemSet::on_enter(AppState::Shop)
                    .with_system(init.system())
            );
    }
}

pub fn init(
    mut commands: Commands,
    handles: Res<Handles>,
) {
    commands
        .spawn_bundle(SpriteBundle {
            material: handles.dummy_card.clone(),
            transform: card_transform(WIDTH / 2. - 200., HEIGHT / 2.),
            ..Default::default()
        })
        .insert(CardComponent { card_id: Cards::DUMMY_1 });

    commands
        .spawn_bundle(SpriteBundle {
            material: handles.dummy_card2.clone(),
            transform: card_transform(WIDTH / 2. + 100., HEIGHT / 2.),
            ..Default::default()
        })
        .insert(CardComponent { card_id: Cards::DUMMY_2 });
}