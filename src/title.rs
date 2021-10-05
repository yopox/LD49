use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel, AudioPlugin};
use rand::prelude::StdRng;
use rand::SeedableRng;

use crate::{AppState, GlobalData, HEIGHT, MySelf, PlayerData, WIDTH};
use crate::card::{BaseCards, Card};
use crate::fight::MyFoe;
use crate::font::TextStyles;
use crate::loading::{AudioAssets, TextureAssets};
use crate::ui::StateBackground;
use crate::util::{Z_BACKGROUND, cleanup_system};

pub struct TitlePlugin;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system_set(
                SystemSet::on_enter(AppState::Title)
                    .with_system(display_title.system())
                    .with_system(setup_data.system())
            )
            .add_system_set(
                SystemSet::on_update(AppState::Title)
                    .with_system(click_to_shop.system())
            )
            .add_system_set(
                SystemSet::on_exit(AppState::Title)
                    .with_system(cleanup_system::<StateBackground>.system())
                    .with_system(cleanup_system::<Title>.system())
            )
        ;
    }
}

struct Title;

fn display_title(
    mut commands: Commands,
    text_styles: Res<TextStyles>,
    handles: Res<TextureAssets>,
    audio: Res<Audio>,
    songs: Res<AudioAssets>,
) {
    audio.stop();
    audio.play_looped(songs.title.clone());

    commands.spawn_bundle(SpriteBundle {
        material: handles.title_bg.clone(),
        transform: Transform {
            translation: Vec3::new(WIDTH / 2., HEIGHT / 2., Z_BACKGROUND),
            ..Default::default()
        },
        ..Default::default()
    }).insert(StateBackground);

    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section("Click to start",
                                 text_styles.subtitle.clone(),
                                 TextAlignment {
                                     horizontal: HorizontalAlign::Center,
                                     ..Default::default()
                                 }),
        transform: Transform {
            translation: Vec3::new(WIDTH / 2., HEIGHT / 2. + 124., 1.),
            ..Default::default()
        },
        ..Default::default()
    }).insert(Title);
}

fn click_to_shop(
    mut app_state: ResMut<State<AppState>>,
    btn: Res<Input<MouseButton>>,
) {
    if btn.just_pressed(MouseButton::Left) {
        app_state.set(AppState::Shop).unwrap();
    }
}

fn setup_data(
    mut commands: Commands,
) {
    commands.spawn().insert(
        PlayerData {
            id: 0,
            name: "YOU".to_string(),
            hand: vec![],
            board: vec![],
            ..Default::default()
        }).insert(MySelf);
    commands.spawn().insert(
        PlayerData {
            id: 1,
            name: "IA".to_string(),
            board: vec![],
            ..Default::default()
        }).insert(MyFoe);

    commands.insert_resource(GlobalData {
        next_card_id: 0, // WARNING: the number of cards created before in this function
        ..Default::default()
    });
}