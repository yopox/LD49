use bevy::prelude::*;

use crate::{AppState, HEIGHT};
use crate::font::TextStyles;

pub struct TitlePlugin;

impl Plugin for TitlePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<ButtonMaterials>()
            .add_system_set(
                SystemSet::on_enter(AppState::Title)
                    .with_system(display_title.system())
            )
            .add_system_set(
                SystemSet::on_update(AppState::Title)
                    .with_system(button_system.system())
            )
            .add_system_set(
                SystemSet::on_exit(AppState::Title)
                    .with_system(recycle_title.system())
            )
        ;
    }
}

struct Title;

struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
    pressed: Handle<ColorMaterial>,
}

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::WHITE.into()),
            hovered: materials.add(Color::GRAY.into()),
            pressed: materials.add(Color::BLACK.into()),
        }
    }
}

fn display_title(
    mut commands: Commands,
    text_styles: Res<TextStyles>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::ColumnReverse,
                position: Rect {
                    top: Val::Px(HEIGHT / 8.),
                    left: Val::Px(0.),
                    ..Default::default()
                },
                size: Size {
                    height: Val::Percent(100.),
                    width: Val::Percent(100.),
                },
                ..Default::default()
            },
            visible: Visible {
                is_transparent: true,
                is_visible: false,
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: "A big title\n\n".to_string(),
                            style: text_styles.bird_seed_title.clone(),
                        }
                    ],
                    ..Default::default()
                },
                visible: Visible {
                    is_visible: true,
                    ..Default::default()
                },
                ..Default::default()
            });

            parent.spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Auto, Val::Auto),
                    margin: Rect::all(Val::Auto),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                visible: Visible { is_transparent: true, is_visible: false },
                ..Default::default()
            })
                .with_children(|parent| {
                    parent.spawn_bundle(TextBundle {
                        text: Text::with_section("Click here to enter", text_styles.bird_seed_subtitle.clone(), Default::default()),
                        ..Default::default()
                    });
                });
        })
        .insert(Title);
}


fn button_system(
    button_materials: Res<ButtonMaterials>,
    mut app_state: ResMut<State<AppState>>,
    mut interaction_query: Query<
        (&Interaction, &mut Handle<ColorMaterial>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut material) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                *material = button_materials.pressed.clone();
                app_state.set(AppState::Shop).unwrap();
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        }
    }
}

fn recycle_title(
    mut commands: Commands,
    query: Query<Entity, With<Title>>,
) {
    let e_id = query.single().expect("There should be one and only one title in the app lifecycle.");
    commands.entity(e_id).despawn_recursive();
}
