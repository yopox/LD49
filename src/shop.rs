use bevy::prelude::*;
use crate::AppState;
use crate::Handles;

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
    commands.spawn_bundle(SpriteBundle {
        material: handles.dummy_card.clone(),
        transform: Transform {
            translation: Vec3::new(0., 0., 1.),
          ..Default::default()
        },
        ..Default::default()
    });
}