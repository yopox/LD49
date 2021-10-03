use bevy::ecs::system::EntityCommands;
use bevy::math::Vec4Swizzles;
use bevy::prelude::*;

use derive_more::Display;

use crate::{Handles, HEIGHT, MainCamera, WIDTH};
use crate::font::*;
use crate::util::{cursor_pos, overlap};

pub const CARD_SCALE: f32 = 0.4;
pub const CARD_WIDTH: f32 = 270. * CARD_SCALE;
pub const CARD_HEIGHT: f32 = 420. * CARD_SCALE;

#[derive(Copy, Clone)]
pub enum CardTypes {
    MUSH_8,
    MERCH_8,
    SPID_8,
    ROB_8,
}

#[derive(Copy, Clone)]
pub struct Card {
    pub card_type: CardTypes,
    pub id: u32,
    pub hp: u16,
    pub at: u16,
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

impl CardTypes {
    pub fn name(&self) -> &'static str {
        match self {
            CardTypes::MUSH_8 => "Titanicus",
            CardTypes::MERCH_8 => "Tujilus",
            CardTypes::SPID_8 => "Australian black widow",
            CardTypes::ROB_8 => "SkyBot",
        }
    }

    pub fn ability(&self) -> Abilities {
        match self {
            CardTypes::MUSH_8 => Abilities::Gigantism,
            CardTypes::MERCH_8 => Abilities::Dexterity,
            CardTypes::SPID_8 => Abilities::Cannibalism,
            CardTypes::ROB_8 => Abilities::Download,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            CardTypes::MUSH_8 => "Gets +1 ATK.",
            CardTypes::MERCH_8 => "Attacks another enemy once.",
            CardTypes::SPID_8 => "Eats the lowest rank spider\nof the board, and gains\nits stats.",
            CardTypes::ROB_8 => "Steals +1 HP and +1 ATK\nfrom each allied robot.",
        }
    }

    pub fn trigger(&self) -> Triggers {
        match self {
            CardTypes::MUSH_8 => Triggers::Kill,
            CardTypes::MERCH_8 => Triggers::Survived,
            CardTypes::SPID_8 => Triggers::Turn,
            CardTypes::ROB_8 => Triggers::Turn,
        }
    }

    pub fn handle(&self, handles: &Res<Handles>) -> Handle<ColorMaterial> {
        match self {
            CardTypes::MUSH_8 => handles.mush_8.clone(),
            CardTypes::MERCH_8 => handles.merch_8.clone(),
            CardTypes::SPID_8 => handles.spid_8.clone(),
            CardTypes::ROB_8 => handles.rob_8.clone(),
        }
    }
}

impl Card {
    pub(crate) fn new(card_type: CardTypes, id: u32) -> Self {
        match card_type {
            CardTypes::MUSH_8 => Card { id, card_type, at: 8, hp: 6 },
            CardTypes::MERCH_8 => Card { id, card_type, at: 5, hp: 9 },
            CardTypes::SPID_8 => Card { id, card_type, at: 4, hp: 4 },
            CardTypes::ROB_8 => Card { id, card_type, at: 3, hp: 3 },
        }
    }
}

pub(crate) struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, init_popup.system())
            .add_system(update_popup.system().after("drag:update"));
    }
}

struct Popup;

const POPUP_X_OFFSET: f32 = 20.;

fn init_popup(
    mut commands: Commands,
    text_styles: Res<TextStyles>,
) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(0.0),
                    left: Val::Px(20.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Name\n\n".to_string(),
                        style: text_styles.love_bug_small.clone(),
                    },
                    TextSection {
                        value: "Ability - Trigger\n\n".to_string(),
                        style: text_styles.bird_seed_small.clone(),
                    },
                    TextSection {
                        value: "Description of the ability.\nCan have multiple lines.".to_string(),
                        style: text_styles.bird_seed_small.clone(),
                    },
                ],
                ..Default::default()
            },
            visible: Visible {
                is_visible: false,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Popup);
}

fn update_popup(
    windows: Res<Windows>,
    mut queries: QuerySet<(
        Query<&Transform, With<MainCamera>>,
        Query<(&Card, &Transform)>,
        Query<(&mut Style, &mut Text, &mut Visible), With<Popup>>,
    )>,
) {
    let mut hover: Option<(CardTypes, Transform)> = None;

    // Get cursor position
    let window = windows.get_primary().unwrap();
    if let Some(cursor) = cursor_pos(window, queries.q0().single().unwrap()) {
        // Get hovered card id & transform
        for (card, transform) in queries.q1().iter() {
            let card_pos = transform.translation;
            if overlap(cursor.xyz(), card_pos, (CARD_WIDTH / 2., CARD_HEIGHT / 2.)) {
                hover = Some((card.card_type.clone(), transform.clone()));
                break;
            }
        }
    }

    // Update popup
    let (mut style, mut text, mut visible) = queries.q2_mut().single_mut().unwrap();
    if let Some((card, transform)) = hover {
        visible.is_visible = true;
        style.position.top = Val::Px(HEIGHT - (transform.translation.y + CARD_HEIGHT / 2.));
        style.position.left = Val::Px(transform.translation.x + CARD_WIDTH / 2. + POPUP_X_OFFSET);
        text.sections[0].value = format!("{}\n\n", card.name().to_string());
        text.sections[1].value = format!("{}\n\n", card.ability().to_string());
        text.sections[2].value = format!("{}\n\n", card.description().to_string());
    } else {
        visible.is_visible = false;
    }
}