use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;

use crate::font::*;
use crate::{MainCamera, WIDTH, HEIGHT, Handles};
use crate::abs::EffectTrigger;
use crate::util::{cursor_pos, overlap};

pub const CARD_SCALE: f32 = 0.4;
pub const CARD_WIDTH: f32 = 270. * CARD_SCALE;
pub const CARD_HEIGHT: f32 = 420. * CARD_SCALE;

#[derive(Copy, Clone)]
pub enum Cards {
    MUSH_8,
    MERCH_8,
    SPID_8,
    ROB_8,
}

pub struct CardComponent {
    pub card_id: Cards,
}

impl Cards {
    pub fn name(&self) -> &'static str {
        match self {
            Cards::MUSH_8 => "Titanicus",
            Cards::MERCH_8 => "Tujilus",
            Cards::SPID_8 => "Australian black widow",
            Cards::ROB_8 => "SkyBot",
        }
    }

    pub fn ability(&self) -> &'static str {
        match self {
            Cards::MUSH_8 => "Gigantism",
            Cards::MERCH_8 => "Dexterity",
            Cards::SPID_8 => "Cannibalism",
            Cards::ROB_8 => "Download",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Cards::MUSH_8 => "Gets +1 ATK.",
            Cards::MERCH_8 => "Attacks another enemy once.",
            Cards::SPID_8 => "Eats the lowest rank spider\nof the board, and gains\nits stats.",
            Cards::ROB_8 => "Steals +1 HP and +1 ATK\nfrom each allied robot.",
        }
    }

    pub fn trigger(&self) -> EffectTrigger {
        match self {
            Cards::MUSH_8 => EffectTrigger::KILL,
            Cards::MERCH_8 => EffectTrigger::SURVIVED,
            Cards::SPID_8 => EffectTrigger::TURN,
            Cards::ROB_8 => EffectTrigger::TURN,
        }
    }

    pub fn handle(&self, handles: &Res<Handles>) -> Handle<ColorMaterial> {
        match self {
            Cards::MUSH_8 => handles.mush_8.clone(),
            Cards::MERCH_8 => handles.merch_8.clone(),
            Cards::SPID_8 => handles.spid_8.clone(),
            Cards::ROB_8 => handles.rob_8.clone(),
        }
    }
}

pub(crate) struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, init_popup.system())
            .add_system(update_popup.system().after("move_card"));
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
        Query<(&CardComponent, &Transform)>,
        Query<(&mut Style, &mut Text, &mut Visible), With<Popup>>,
    )>,
) {
    let mut hover: Option<(Cards, Transform)> = None;

    // Get cursor position
    let window = windows.get_primary().unwrap();
    if let Some(cursor) = cursor_pos(window, queries.q0().single().unwrap()) {
        // Get hovered card id & transform
        for (card, transform) in queries.q1().iter() {
            let card_pos = transform.translation;
            if overlap(cursor, card_pos, (CARD_WIDTH / 2., CARD_HEIGHT / 2.)) {
                hover = Some((card.card_id.clone(), transform.clone()));
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