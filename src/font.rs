use bevy::prelude::*;

pub struct TextStyles {
    pub bird_seed_small: TextStyle,
    pub bird_seed_title: TextStyle,
    pub love_bug_small: TextStyle,
    pub bird_seed_subtitle: TextStyle,
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
            font_size: 20.0,
            color: Color::BLACK
        },
        love_bug_small: TextStyle {
            font: love_bug.clone(),
            font_size: 18.0,
            color: Color::BLACK
        },
        bird_seed_title: TextStyle {
            font: bird_seed.clone(),
            font_size: 200.0,
            color: Color::BLACK,
        },
        bird_seed_subtitle: TextStyle {
            font: bird_seed.clone(),
            font_size: 60.0,
            color: Color::BLACK,
        }
    });
}