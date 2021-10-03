mod shop;
mod abs;
mod font;
mod card;
mod util;
mod title;
mod anim;

use bevy::prelude::*;
use crate::shop::ShopPlugin;
use crate::card::CardPlugin;
use crate::title::TitlePlugin;
use crate::anim::AnimationPlugin;

pub const WIDTH: f32 = 1280.;
pub const HEIGHT: f32 = 720.;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Title,
    Shop,
    Fight,
}

pub struct Handles {
    pub mush_8: Handle<ColorMaterial>,
    pub merch_8: Handle<ColorMaterial>,
    pub spid_8: Handle<ColorMaterial>,
    pub rob_8: Handle<ColorMaterial>,
}
struct MainCamera;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "LD49".to_string(),
            width: WIDTH,
            height: HEIGHT,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShopPlugin)
        .add_plugin(CardPlugin)
        .add_state(AppState::Shop)
        .add_plugin(AnimationPlugin)
        .add_plugin(TitlePlugin)
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
        mush_8: materials.add(asset_server.load("MUSH_8.png").into()),
        merch_8: materials.add(asset_server.load("MERCH_8.png").into()),
        spid_8: materials.add(asset_server.load("SPID_8.png").into()),
        rob_8: materials.add(asset_server.load("ROB_8.png").into()),
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