use std::collections::HashMap;
use bevy::app::{AppBuilder, EventReader, Plugin};
use bevy::ecs::entity::Entity;
use bevy::ecs::prelude::*;
use bevy::ecs::system::{Commands, Query, QuerySet};
use bevy::math::{Size, Vec2, Vec3, Vec4Swizzles};
use bevy::prelude::{Children, HorizontalAlign, Parent, Sprite, SpriteBundle, Text, Text2dBundle, TextAlignment};
use bevy::render::draw::Visible;
use bevy::text::{Text2dSize, TextSection};
use bevy::transform::components::Transform;
use bevy::transform::hierarchy::BuildChildren;
use bevy::window::Windows;
use crate::data::card::{Abilities, Card, CARD_HEIGHT, CARD_SCALE, CARD_WIDTH};
use crate::{AppState, MainCamera};
use crate::data::font::TextStyles;
use crate::data::loading::{ColorAssets, TextureAssets};
use crate::ui::drag_and_drop::Dragged;
use crate::util::{cursor_pos, overlap, Z_POPUP_BG, Z_POPUP_TEXT, Z_STATS};

pub struct Popup;

pub struct PopupBackground;

struct AtkStat;

struct HpStat;

const POPUP_PADDING: f32 = 10.;

pub(crate) struct CardPlugin;

pub struct NewCard(pub Entity, pub Card);

pub struct StatsChanged(pub Entity);

pub struct Prepare;

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
            )
            .add_system_set(
                SystemSet::on_update(AppState::GameOver)
                    .with_system(init_popup.system().label("popup:init"))
                    .with_system(update_size.system().label("popup:update").after("popup:init"))
                    .with_system(update_popup_visibility.system().after("popup:update"))
            );
    }
}

pub(crate) fn init_popup(
    mut commands: Commands,
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
                                value: format!("{}\n", base_card.name().to_string()),
                                style: text_styles.love_bug_small.clone(),
                            },
                            TextSection {
                                value: format!("Level {}\n\n", base_card.rank()),
                                style: text_styles.bird_seed_small.clone(),
                            },
                            TextSection {
                                value: if base_card.ability() != Abilities::None {
                                    format!("Ability: {}\nTriggered on {}\n\n", base_card.ability().to_string(), base_card.trigger().to_string())
                                } else { "".to_string() },
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
                .spawn_bundle(Text2dBundle {
                    text: Text::with_section(format!("{}", new_card.1.hp),
                                             text_styles.stats.clone(),
                                             TextAlignment {
                                                 horizontal: HorizontalAlign::Center,
                                                 ..Default::default()
                                             }),
                    transform: Transform {
                        translation: Vec3::new(CARD_WIDTH / 4. / CARD_SCALE + 10., -CARD_HEIGHT / 2. / CARD_SCALE + 23., Z_STATS),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(HpStat);

            parent
                .spawn_bundle(Text2dBundle {
                    text: Text::with_section(format!("{}", new_card.1.atk),
                                             text_styles.stats.clone(),
                                             TextAlignment {
                                                 horizontal: HorizontalAlign::Center,
                                                 ..Default::default()
                                             }),
                    transform: Transform {
                        translation: Vec3::new(-CARD_WIDTH / 4. / CARD_SCALE + 32., -CARD_HEIGHT / 2. / CARD_SCALE + 23., Z_STATS),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(AtkStat);
        });
    }
}

pub(crate) fn update_popup_visibility(
    windows: Res<Windows>,
    mut queries: QuerySet<(
        Query<&Transform, With<MainCamera>>,
        Query<(Entity, &Transform), (With<Dragged>, With<Card>)>,
        Query<(Entity, &Transform, &Children), With<Card>>,
    )>,
    mut visible_queries: QuerySet<(
        Query<&mut Visible, With<Popup>>,
        Query<&mut Visible, With<PopupBackground>>,
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

pub(crate) fn update_size(
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
                    text.sections[0].value = format!("{}", card.atk);
                    break;
                }
            }

            for (parent, mut text) in texts.q1_mut().iter_mut() {
                if parent.0 == event.0 {
                    text.sections[0].value = format!("{}", card.hp);
                    break;
                }
            }
        }
    }
}