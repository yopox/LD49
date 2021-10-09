use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel};
use rand::Rng;
use rand::distributions::Standard;

use crate::{AppState, GlobalData, HEIGHT, MySelf, PlayerData, WIDTH};
use crate::fight::fight_screen::MyFoe;
use crate::data::font::TextStyles;
use crate::data::loading::{AudioAssets, TextureAssets};
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
    audio.set_volume_in_channel(0.8, &AudioChannel::new("SFX".to_owned()));
    audio.set_volume(0.8);
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
            translation: Vec3::new(WIDTH / 2., HEIGHT / 2. + 120., 1.),
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
    let mut global_data = GlobalData {
        next_card_id: 0, // WARNING: the number of cards created before in this function
        ..Default::default()
    };
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
            name: "AI".to_string(),
            board: vec![],
            ia: global_data.rng.sample(Standard),
            ..Default::default()
        }).insert(MyFoe);

    commands.insert_resource(global_data);
}