use bevy::prelude::*;
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

use crate::card::{Card, CardPlugin, CardsID};
use crate::fight::{FightPlugin, MyFoe};
use crate::shop::ShopPlugin;
use crate::title::TitlePlugin;
use crate::ui::{AnimationPlugin, DragAndDropPlugin};

mod shop;
mod abs;
mod font;
mod card;
mod util;
mod title;
mod ui;
mod fight;

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

    pub shop_bg: Handle<ColorMaterial>,
    pub slot_border: Handle<ColorMaterial>,
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
        .add_plugin(DragAndDropPlugin)
        .add_plugin(FightPlugin)
        .add_plugin(TitlePlugin)
        .add_startup_system(setup.system())
        .add_startup_system(crate::font::load_fonts.system())
        .add_startup_system(setup_data.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Load assets
    commands.insert_resource(Handles {
        mush_8: materials.add(asset_server.load("MUSH_8.png").into()),
        merch_8: materials.add(asset_server.load("MERCH_8.png").into()),
        spid_8: materials.add(asset_server.load("SPID_8.png").into()),
        rob_8: materials.add(asset_server.load("ROB_8.png").into()),

        shop_bg: materials.add(asset_server.load("shop.png").into()),
        slot_border: materials.add(asset_server.load("slot_border.png").into()),
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

pub struct MySelf;

#[derive(Clone)]
pub struct PlayerData {
    id: u16,
    name: String,
    hand: Vec<Card>,
    board: Vec<Card>,
    coins: u16,
    hp: u16,
    shop_level: u16,
    // hero,
}

impl Default for PlayerData {
    fn default() -> Self {
        PlayerData {
            id: 0,
            name: "J1".to_string(),
            hand: vec![],
            board: vec![],
            coins: 3,
            hp: 25,
            shop_level: 1,
        }
    }
}

pub struct GlobalData {
    rng: StdRng,
    turn: u16,
    // Nothing for now
}

fn setup_data(
    mut commands: Commands,
) {
    commands.insert_resource(GlobalData {
        rng: StdRng::seed_from_u64(0u64),
        turn: 0,
    });

    commands.spawn().insert(
        PlayerData {
            id: 0,
            name: "H".to_string(),
            hand: vec![
                Card::from(CardsID::SPID_8),
            ],
            board: vec![
                Card::from(CardsID::ROB_8),
                Card::from(CardsID::MUSH_8),
            ],
            ..Default::default()
        }).insert(MySelf);
    commands.spawn().insert(
        PlayerData {
            id: 1,
            name: "L".to_string(),
            board: vec![
                Card::from(CardsID::SPID_8),
                Card::from(CardsID::MERCH_8),
            ],
            ..Default::default()
        }).insert(MyFoe);
}
