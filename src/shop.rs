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
            transform: Transform {
                translation: Vec3::new(WIDTH / 2. - 100., HEIGHT / 2., 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(CardComponent { card_id: Cards::DUMMY_1 });

    commands
        .spawn_bundle(SpriteBundle {
            material: handles.dummy_card2.clone(),
            transform: Transform {
                translation: Vec3::new(WIDTH / 2. + 100., HEIGHT / 2., 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(CardComponent { card_id: Cards::DUMMY_2 });
}