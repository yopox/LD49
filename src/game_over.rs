use bevy::prelude::*;
use bevy_kira_audio::Audio;

use crate::{AppState, HEIGHT, PlayerData, WIDTH, MySelf};
use crate::fight::fight_screen::FightBackup;
use crate::data::font::TextStyles;
use crate::data::loading::{AudioAssets, TextureAssets};
use crate::ui::StateBackground;
use crate::ui::card_overlay::NewCard;
use crate::util::{Z_BACKGROUND, cleanup_system, Slot, card_transform};

#[derive(PartialEq, Copy, Clone)]
struct GameOverSlot {
    id: u8,
    total: u8,
}

impl Slot for GameOverSlot {
    fn x(&self) -> f32 { return 256. + 128. * self.id as f32 + 64. * (7 - self.total) as f32; }
    fn y(&self) -> f32 { return HEIGHT / 2.; }
}

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_system_set(
                SystemSet::on_enter(AppState::GameOver)
                    .with_system(init.system())
            )
            .add_system_set(
                SystemSet::on_update(AppState::GameOver)
                    .with_system(click_to_title.system())
            )
            .add_system_set(
                SystemSet::on_exit(AppState::GameOver)
                    .with_system(cleanup_system::<StateBackground>.system())
                    .with_system(cleanup_system::<Over>.system())
                    .with_system(cleanup_system::<PlayerData>.system())
            )
        ;
    }
}

struct Over;

pub struct Won(pub bool);

fn init(
    mut commands: Commands,
    text_styles: Res<TextStyles>,
    mut player_data: Query<&PlayerData, With<MySelf>>,
    mut ev_card: EventWriter<NewCard>,
    won: Res<Won>,
    handles: Res<TextureAssets>,
    audio: Res<Audio>,
    songs: Res<AudioAssets>,
) {
    audio.stop();
    audio.play_looped(songs.title.clone());

    commands.spawn_bundle(SpriteBundle {
        material: handles.fight_bg.clone(),
        transform: Transform {
            translation: Vec3::new(WIDTH / 2., HEIGHT / 2., Z_BACKGROUND),
            ..Default::default()
        },
        ..Default::default()
    }).insert(StateBackground);

    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section("Click to return to the title screen",
                                 text_styles.subtitle.clone(),
                                 TextAlignment {
                                     horizontal: HorizontalAlign::Center,
                                     ..Default::default()
                                 }),
        transform: Transform {
            translation: Vec3::new(WIDTH / 2., HEIGHT / 4., 1.),
            ..Default::default()
        },
        ..Default::default()
    }).insert(Over);

    commands.spawn_bundle(Text2dBundle {
        text: Text::with_section(if won.0 { "You won!" } else { "You lost!" },
                                 text_styles.subtitle.clone(),
                                 TextAlignment {
                                     horizontal: HorizontalAlign::Center,
                                     ..Default::default()
                                 }),
        transform: Transform {
            translation: Vec3::new(WIDTH / 2., 3. * HEIGHT / 4., 1.),
            ..Default::default()
        },
        ..Default::default()
    }).insert(Over);

    let player_data = player_data.single_mut().unwrap();
    let mut nb_cards = 0;
    for _ in player_data.board.iter() { nb_cards += 1; }
    
    for (i, card) in player_data.board.iter().enumerate() {
        let slot = GameOverSlot { id: i as u8, total: nb_cards };
        let e = commands
            .spawn_bundle(SpriteBundle {
                material: card.base_card.handle(&handles),
                transform: card_transform(slot.x(), slot.y()),
                ..Default::default()
            })
            .insert(card.clone())
            .insert(slot)
            .insert(Over)
            .id();
        ev_card.send(NewCard(e, card.clone()));
    }
}

fn click_to_title(
    mut app_state: ResMut<State<AppState>>,
    btn: Res<Input<MouseButton>>,
) {
    if btn.just_released(MouseButton::Left) {
        app_state.set(AppState::Title).unwrap();
    }
}
