mod shop;
mod abs;
mod font;
mod card;
mod util;

use bevy::prelude::*;
use crate::shop::ShopPlugin;
use crate::card::CardPlugin;

pub const WIDTH: f32 = 1280.;
pub const HEIGHT: f32 = 720.;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Title,
    Shop,
    Fight,
}

pub struct Handles {
    pub dummy_card: Handle<ColorMaterial>,
    pub dummy_card2: Handle<ColorMaterial>,
}
struct MainCamera;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(ShopPlugin)
        .add_plugin(CardPlugin)
        .insert_resource(WindowDescriptor {
            title: "LD49".to_string(),
            width: WIDTH,
            height: HEIGHT,
            vsync: true,
            ..Default::default()
        })
        .add_state(AppState::Shop)
        .add_startup_system(setup.system())
        .add_startup_system(crate::font::load_fonts.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Load assets
    commands.insert_resource(Handles {
        dummy_card: materials.add(asset_server.load("dummy_card.png").into()),
        dummy_card2: materials.add(asset_server.load("dummy_card2.png").into()),
    });

    // Spawn camera
    let mut camera = OrthographicCameraBundle::new_2d();
    camera.transform = Transform {
        translation: Vec3::new(WIDTH / 2., HEIGHT / 2., 999.),
        ..Default::default()
    };
    commands
        .spawn_bundle(camera)
        .insert(MainCamera);
    commands.spawn_bundle(UiCameraBundle::default());
}