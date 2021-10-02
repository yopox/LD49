use bevy::prelude::*;

use crate::font::*;
use crate::{MainCamera, WIDTH, HEIGHT};
use crate::util::cursor_pos;

pub const CARD_SCALE: f32 = 0.4;
const CARD_WIDTH: f32 = 270. * CARD_SCALE;
const CARD_HEIGHT: f32 = 420. * CARD_SCALE;

pub fn card_transform(x: f32, y: f32) -> Transform {
    return Transform {
        translation: Vec3::new(x, y, 2.),
        scale: Vec3::new(CARD_SCALE, CARD_SCALE, 1.),
        ..Default::default()
    }
}

#[derive(Copy, Clone)]
pub enum Cards {
    DUMMY_1,
    DUMMY_2,
}

pub struct CardComponent {
    pub card_id: Cards,
}

impl Cards {
    fn name(&self) -> &'static str {
        match self {
            Cards::DUMMY_1 => "Dummy",
            Cards::DUMMY_2 => "Dummy 2",
        }
    }

    fn ability(&self) -> &'static str {
        match self {
            Cards::DUMMY_1 => "Ability 1",
            Cards::DUMMY_2 => "Ability 2",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            Cards::DUMMY_1 => "Dummy description :-)",
            Cards::DUMMY_2 => "Another dummy description :-(",
        }
    }
}

pub(crate) struct CardPlugin;

impl Plugin for CardPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system_to_stage(StartupStage::PostStartup, init_popup.system())
            .add_system(update_popup.system());
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
            if cursor.x >= card_pos.x - CARD_WIDTH / 2. && cursor.x < card_pos.x + CARD_WIDTH / 2. &&
                cursor.y >= card_pos.y - CARD_HEIGHT / 2. && cursor.y < card_pos.y + CARD_HEIGHT / 2. {
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