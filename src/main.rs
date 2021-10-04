use bevy::prelude::*;
use bevy_asset_loader::AssetLoader;
use bevy_kira_audio::{AudioChannel, Audio, AudioPlugin, AudioSource};
use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

use crate::card::{Card, CardPlugin, BaseCards};
use crate::fight::{FightPlugin, MyFoe};
use crate::loading::{AudioAssets, ColorAssets, TextureAssets};
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
mod loading;
mod shop_rules;
mod shop_manager;

pub const WIDTH: f32 = 1280.;
pub const HEIGHT: f32 = 720.;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Loading,
    Title,
    Shop,
    Fight,
}

struct MainCamera;

fn main() {
    let mut app = App::build();
    AssetLoader::new(AppState::Loading, AppState::Title)
        .with_collection::<TextureAssets>()
        .with_collection::<AudioAssets>()
        .build(&mut app);
    app
        .add_state(AppState::Loading)
        .insert_resource(WindowDescriptor {
            title: "LD49".to_string(),
            width: WIDTH,
            height: HEIGHT,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(AudioPlugin)
        .add_plugin(ShopPlugin)
        .add_plugin(CardPlugin)
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
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
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

    commands.insert_resource(ColorAssets {
        background: materials.add(Color::rgb(244. / 255., 237. / 255., 219. / 255.).into()),
        black: materials.add(Color::rgb(0., 0., 0.).into()),
    });
}

pub struct MySelf;

#[derive(Clone)]
pub struct PlayerData {
    id: u16,
    name: String,
    hand: Vec<Card>,
    board: Vec<Card>,
    coins: u16,
    extra_coins: u16, // For gold gained in fight
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
            extra_coins: 0,
            hp: 25,
            shop_level: 1,
        }
    }
}

pub struct GlobalData {
    rng: StdRng,
    turn: u16,
    next_card_id: u32,
}

fn setup_data(
    mut commands: Commands,
) {
    commands.spawn().insert(
        PlayerData {
            id: 0,
            name: "H".to_string(),
            hand: vec![
                Card::new(BaseCards::SPID_8, 0),
            ],
            board: vec![
                Card::new(BaseCards::ROB_8, 1),
                Card::new(BaseCards::MUSH_8, 2),
            ],
            ..Default::default()
        }).insert(MySelf);
    commands.spawn().insert(
        PlayerData {
            id: 1,
            name: "L".to_string(),
            board: vec![
                Card::new(BaseCards::SPID_8, 3),
                Card::new(BaseCards::MERCH_8, 4),
            ],
            ..Default::default()
        }).insert(MyFoe);

    commands.insert_resource(GlobalData {
        rng: StdRng::seed_from_u64(0u64),
        turn: 0,
        next_card_id: 5, // WARNING: the number of cards created before in this function
    });
}
