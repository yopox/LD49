use bevy::prelude::*;

use crate::font::*;

#[derive(Copy, Clone)]
pub enum Cards {
    DUMMY,
}

pub struct CardComponent {
    pub card_id: Cards,
}

impl Cards {
    fn name(&self) -> &'static str {
        match self {
            Cards::DUMMY => "Dummy",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            Cards::DUMMY => "Dummy description :-)",
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

fn init_popup(
    mut commands: Commands,
    text_styles: Res<TextStyles>,
) {
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
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
            // visible: Visible {
            //     is_visible: true,
            //     ..Default::default()
            // },
            ..Default::default()
        })
        .insert(Popup);
}

fn update_popup(
    mut queries: QuerySet<(
        Query<(&CardComponent, &Transform)>,
        Query<&mut TextBundle, With<Popup>>,
    )>,
) {}