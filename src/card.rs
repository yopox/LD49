use std::collections::HashMap;
use std::error::Error;

use bevy::ecs::system::EntityCommands;
use bevy::math::Vec4Swizzles;
use bevy::prelude::*;
use bevy::text::Text2dSize;
use derive_more::Display;

use crate::{HEIGHT, WIDTH, MainCamera, AppState};
use crate::loading::{ColorAssets, TextureAssets};
use crate::font::*;
use crate::ui::Dragged;
use crate::util::{cursor_pos, overlap, Z_POPUP_BG, Z_POPUP_TEXT, Z_STATS, Z_STATS_BG};

pub const CARD_SCALE: f32 = 0.4;
pub const CARD_WIDTH: f32 = 270. * CARD_SCALE;
pub const CARD_HEIGHT: f32 = 420. * CARD_SCALE;

#[derive(Copy, Clone)]
pub enum BaseCards {
    MUSH_8,
    MERCH_8,
    SPID_8,
    ROB_8,
}

#[derive(Copy, Clone)]
pub struct Card {
    pub base_card: BaseCards,
    pub id: u32,
    pub hp: u16,
    pub atk: u16,
    pub played: u8,
}

impl Default for Card {
    fn default() -> Self {
        Card {
            base_card: BaseCards::MERCH_8,
            id: 0,
            hp: 0,
            atk: 0,
            played: 0,
        }
    }
}

#[derive(Debug, Display, PartialEq, Eq, Copy, Clone)]
pub enum Abilities {
    // Mushrooms
    ToxicSpores,
    Gigantism,
    // Merchants
    Sadism,
    ExplodingArmour,
    Pillage,
    Dexterity,
    // Spiders
    Trap,
    Multiplication,
    Poisonous,
    Cannibalism,
    // Robots
    Download,

    // None or not at combat time
    None,
}

#[derive(Debug, Display, PartialEq, Eq, Copy, Clone)]
pub enum Triggers {
    // At the beginning of each turn
    Turn,
    // When this card is played
    Played,
    // When this card dies
    Death,
    // When this card attacks and survives
    Survived,
    // When this card attacks or is attacked
    Hit,
    // When this card kills
    Kill,
    // When this card is sold
    Sold,
    // PASSIVE,
    None,
}

impl BaseCards {
    pub fn name(&self) -> &'static str {
        match self {
            BaseCards::MUSH_8 => "Titanicus",
            BaseCards::MERCH_8 => "Tujilus",
            BaseCards::SPID_8 => "Australian black widow",
            BaseCards::ROB_8 => "SkyBot",
        }
    }

    pub fn ability(&self) -> Abilities {
        match self {
            BaseCards::MUSH_8 => Abilities::Gigantism,
            BaseCards::MERCH_8 => Abilities::Dexterity,
            BaseCards::SPID_8 => Abilities::Cannibalism,
            BaseCards::ROB_8 => Abilities::Download,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            BaseCards::MUSH_8 => "Gets +1 ATK.",
            BaseCards::MERCH_8 => "Attacks another enemy once.",
            BaseCards::SPID_8 => "Eats the lowest rank spider\nof the board, and gains\nits stats.",
            BaseCards::ROB_8 => "Steals +1 HP and +1 ATK\nfrom each allied robot.",
        }
    }

    pub fn rank(&self) -> u8 {
        match self {
            BaseCards::MUSH_8 => 4,
            BaseCards::MERCH_8 => 4,
            BaseCards::SPID_8 => 4,
            BaseCards::ROB_8 => 4,
        }
    }

    pub fn trigger(&self) -> Triggers {
        match self {
            BaseCards::MUSH_8 => Triggers::Kill,
            BaseCards::MERCH_8 => Triggers::Survived,
            BaseCards::SPID_8 => Triggers::Turn,
            BaseCards::ROB_8 => Triggers::Turn,
        }
    }

    pub fn handle(&self, handles: &Res<TextureAssets>) -> Handle<ColorMaterial> {
        match self {
            BaseCards::MUSH_8 => handles.mush_8.clone(),
            BaseCards::MERCH_8 => handles.merch_8.clone(),
            BaseCards::SPID_8 => handles.spid_8.clone(),
            BaseCards::ROB_8 => handles.rob_8.clone(),
        }
    }
}

impl Card {
    pub(crate) fn new(card_type: BaseCards, id: u32) -> Self {
        match card_type {
            BaseCards::MUSH_8 => Card { id, base_card: card_type, atk: 8, hp: 6, ..Default::default() },
            BaseCards::MERCH_8 => Card { id, base_card: card_type, atk: 5, hp: 9, ..Default::default() },
            BaseCards::SPID_8 => Card { id, base_card: card_type, atk: 4, hp: 4, ..Default::default() },
            BaseCards::ROB_8 => Card { id, base_card: card_type, atk: 3, hp: 3, ..Default::default() },
        }
    }
}

pub(crate) struct CardPlugin;

pub struct NewCard(pub Entity, pub Card);

pub struct StatsChanged(pub Entity);

struct Prepare;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_event::<NewCard>()
            .add_event::<StatsChanged>()
            .add_system_set(
                SystemSet::on_update(AppState::Shop)
                    .with_system(init_popup.system().label("popup:init"))
                    .with_system(update_size.system().label("popup:update").after("popup:init"))
                    .with_system(update_popup_visibility.system().after("popup:update"))
                    .with_system(update_stats.system())
            )
            .add_system_set(
                SystemSet::on_update(AppState::Fight)
                    .with_system(init_popup.system().label("popup:init"))
                    .with_system(update_size.system().label("popup:update").after("popup:init"))
                    .with_system(update_popup_visibility.system().after("popup:update"))
                    .with_system(update_stats.system())
            );
    }
}

struct Popup;
struct PopupBackground;
struct AtkStat;
struct HpStat;
struct StatsBackground;

const POPUP_X_OFFSET: f32 = 20.;
const POPUP_PADDING: f32 = 10.;

fn init_popup(
    mut commands: Commands,
    handles: Res<TextureAssets>,
    colors: Res<ColorAssets>,
    mut ev_new_card: EventReader<NewCard>,
    text_styles: Res<TextStyles>,
) {
    for new_card in ev_new_card.iter() {
        let base_card = new_card.1.base_card;
        commands.entity(new_card.0).with_children(|parent| {
            parent
                .spawn_bundle(Text2dBundle {
                    text: Text {
                        sections: vec![
                            TextSection {
                                value: format!("{}\n\n", base_card.name().to_string()),
                                style: text_styles.love_bug_small.clone(),
                            },
                            TextSection {
                                value: format!("{}\n\n", base_card.ability().to_string()),
                                style: text_styles.bird_seed_small.clone(),
                            },
                            TextSection {
                                value: format!("{}\n\n", base_card.description().to_string()),
                                style: text_styles.bird_seed_small.clone(),
                            },
                        ],
                        ..Default::default()
                    },
                    visible: Visible {
                        is_visible: false,
                        is_transparent: true,
                    },
                    transform: Transform {
                        translation: Vec3::new(0.0, 0.0, Z_POPUP_TEXT),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(Popup)
                .insert(Prepare);

            parent
                .spawn_bundle(SpriteBundle {
                    material: colors.background.clone(),
                    sprite: Sprite::new(Vec2::new(0.0, 0.0)),
                    visible: Visible {
                        is_visible: false,
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: Vec3::new(0., 0., Z_POPUP_BG),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(PopupBackground)
                .insert(Prepare);

            parent
                .spawn_bundle(SpriteBundle {
                    material: colors.background.clone(),
                    sprite: Sprite::new(Vec2::new(254.0, 50.0)),
                    transform: Transform {
                        translation: Vec3::new(0., (-CARD_HEIGHT / 2. + 12.) / CARD_SCALE, Z_STATS_BG),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(StatsBackground);

            parent
                .spawn_bundle(Text2dBundle {
                    text: Text::with_section(format!("{} HP", new_card.1.hp),
                                             text_styles.stats.clone(),
                                             TextAlignment {
                                                 horizontal: HorizontalAlign::Center,
                                                 ..Default::default()
                                             }),
                    transform: Transform {
                        translation: Vec3::new(-CARD_WIDTH / 4. / CARD_SCALE + 6., -CARD_HEIGHT / 2. / CARD_SCALE + 10., Z_STATS),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(HpStat);

            parent
                .spawn_bundle(Text2dBundle {
                    text: Text::with_section(format!("{} ATK", new_card.1.atk),
                                             text_styles.stats.clone(),
                                             TextAlignment {
                                                 horizontal: HorizontalAlign::Center,
                                                 ..Default::default()
                                             }),
                    transform: Transform {
                        translation: Vec3::new(CARD_WIDTH / 4. / CARD_SCALE - 6., -CARD_HEIGHT / 2. / CARD_SCALE + 10., Z_STATS),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(HpStat);
        });
    }
}

fn update_popup_visibility(
    windows: Res<Windows>,
    mut queries: QuerySet<(
        Query<&Transform, With<MainCamera>>,
        Query<(Entity, &Transform), (With<Dragged>, With<Card>)>,
        Query<(Entity, &Transform, &Children), With<Card>>,
    )>,
    mut visible_queries: QuerySet<(
        Query<(&mut Visible), With<Popup>>,
        Query<(&mut Visible), With<PopupBackground>>,
    )>,
) {
    // Get cursor position
    let window = windows.get_primary().unwrap();
    if let Some(cursor) = cursor_pos(window, queries.q0().single().unwrap()) {
        // If a card is dragged and hovered it has popup priority
        let mut priority: Option<Entity> = None;
        for (e, transform) in queries.q1().iter() {
            let card_pos = transform.translation;
            if overlap(cursor.xyz(), card_pos, (CARD_WIDTH / 2., CARD_HEIGHT / 2.)) {
                priority = Some(e);
                break;
            }
        }

        // Set visibilities
        for (e, transform, children) in queries.q2().iter() {
            let visible = match priority {
                Some(dragged_e) => e == dragged_e,
                None => overlap(cursor.xyz(), transform.translation, (CARD_WIDTH / 2., CARD_HEIGHT / 2.)),
            };
            for child in children.iter() {
                if let Ok(mut visible_text) = visible_queries.q0_mut().get_mut(*child) {
                    visible_text.is_visible = visible;
                    continue;
                }
                if let Ok(mut visible_bg) = visible_queries.q1_mut().get_mut(*child) {
                    visible_bg.is_visible = visible;
                }
            }
        }
    }
}

fn update_size(
    mut commands: Commands,
    mut queries: QuerySet<(
        Query<(Entity, &Parent, &mut Transform, &Text2dSize), (With<Popup>, With<Prepare>)>,
        Query<(Entity, &Parent, &mut Sprite, &mut Transform), (With<PopupBackground>, With<Prepare>)>,
    )>,
) {
    let mut text_sizes: HashMap<Entity, Size> = HashMap::new();
    for (text_e, parent, mut transform, text_size) in queries.q0_mut().iter_mut() {
        let size = text_size.size;
        if size.width == 0. && size.height == 0. { break; }
        transform.translation.x = (CARD_WIDTH / 2. + size.width + POPUP_PADDING) / CARD_SCALE;
        transform.translation.y = (CARD_HEIGHT / 2. - size.height - POPUP_PADDING) / CARD_SCALE;
        commands.entity(text_e).remove::<Prepare>();
        text_sizes.insert(parent.0, size.clone());
    }

    for (bg_e, bg_parent, mut sprite, mut bg_transform) in queries.q1_mut().iter_mut() {
        if let Some(size) = text_sizes.get(&bg_parent.0) {
            sprite.size.x = (&size.width + 2. * POPUP_PADDING) / CARD_SCALE;
            sprite.size.y = (&size.height + 2. * POPUP_PADDING) / CARD_SCALE;
            bg_transform.translation.x = (CARD_WIDTH / 2. + &size.width / 2. + POPUP_PADDING) / CARD_SCALE;
            bg_transform.translation.y = (CARD_HEIGHT / 2. - &size.height / 2. - POPUP_PADDING) / CARD_SCALE;
            commands.entity(bg_e).remove::<Prepare>();
        }
    }
}

fn update_stats(
    mut ev_stats: EventReader<StatsChanged>,
    mut texts: QuerySet<(
        Query<(&Parent, &mut Text), With<AtkStat>>,
        Query<(&Parent, &mut Text), With<HpStat>>,
    )>,
    cards: Query<&Card>,
) {
    for event in ev_stats.iter() {
        if let Ok(card) = cards.get(event.0) {
            for (parent, mut text) in texts.q0_mut().iter_mut() {
                if parent.0 == event.0 {
                    text.sections[0].value = format!("{} ATK", card.atk);
                    break;
                }
            }

            for (parent, mut text) in texts.q1_mut().iter_mut() {
                if parent.0 == event.0 {
                    text.sections[0].value = format!("{} HP", card.hp);
                    break;
                }
            }
        }
    }
}