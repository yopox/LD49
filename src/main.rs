use bevy::prelude::*;
use bevy_asset_loader::AssetLoader;
use bevy_kira_audio::AudioPlugin;
use rand::SeedableRng;
use rand::rngs::StdRng;

#[cfg(target_arch = "wasm32")]
use bevy_webgl2;

use crate::data::card::Card;
use crate::fight::fight_screen::FightPlugin;
use crate::game_over::GameOverPlugin;
use crate::data::loading::{AudioAssets, ColorAssets, TextureAssets};
use crate::fight::predefined_hands::HandsName;
use crate::shop::shop_screen::ShopPlugin;
use crate::title::TitlePlugin;
use crate::ui::card_overlay::CardPlugin;
use crate::ui::drag_and_drop::DragAndDropPlugin;
use crate::ui::transition::AnimationPlugin;

mod util;
mod title;
mod game_over;
mod data;
mod fight;
mod shop;
mod ui;

pub const WIDTH: f32 = 1280.;
pub const HEIGHT: f32 = 720.;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Loading,
    Title,
    Shop,
    Fight,
    GameOver,
}

struct MainCamera;

fn main() {
    let mut app = App::build();

    app.add_plugins(DefaultPlugins);
    
    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    AssetLoader::new(AppState::Loading, AppState::Title)
        .with_collection::<TextureAssets>()
        .with_collection::<AudioAssets>()
        .build(&mut app);

    app
        .add_state(AppState::Loading)
        .insert_resource(WindowDescriptor {
            title: "Unbalanced Brawl".to_string(),
            width: WIDTH,
            height: HEIGHT,
            vsync: true,
            ..Default::default()
        })
        .add_plugin(AudioPlugin)
        .add_plugin(ShopPlugin)
        .add_plugin(CardPlugin)
        .add_plugin(AnimationPlugin)
        .add_plugin(DragAndDropPlugin)
        .add_plugin(FightPlugin)
        .add_plugin(TitlePlugin)
        .add_plugin(GameOverPlugin)
        .add_startup_system(setup.system())
        .add_startup_system(crate::data::font::load_fonts.system())
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
    ia: HandsName,
    // hero,
}

impl Default for PlayerData {
    fn default() -> Self {
        PlayerData {
            id: 0,
            name: "YOU".to_string(),
            hand: vec![],
            board: vec![],
            coins: 3,
            extra_coins: 0,
            hp: 25,
            shop_level: 1,
            ia: HandsName::Mush,
        }
    }
}

pub struct GlobalData {
    rng: StdRng,
    turn: u16,
    next_card_id: u32,
}

impl Default for GlobalData {
    fn default() -> Self {
        GlobalData {
            rng: StdRng::from_entropy(),
            turn: 0,
            next_card_id: 0,
        }
    }
}
