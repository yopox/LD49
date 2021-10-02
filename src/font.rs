use bevy::prelude::*;

pub struct TextStyles {
    pub bird_seed_small: TextStyle,
    pub love_bug_small: TextStyle,
}

pub fn load_fonts(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let bird_seed: Handle<Font> = asset_server.load("ChevyRay - Bird Seed.ttf");
    let love_bug: Handle<Font> = asset_server.load("ChevyRay - Love Bug.ttf");
    commands.insert_resource(TextStyles {
        bird_seed_small: TextStyle {
            font: bird_seed.clone(),
            font_size: 10.0,
            color: Color::WHITE
        },
        love_bug_small: TextStyle {
            font: love_bug.clone(),
            font_size: 9.0,
            color: Color::WHITE
        },
    });
}