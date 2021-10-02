use bevy::prelude::*;

const WIDTH: f32 = 1280.;
const HEIGHT: f32 = 720.;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "LD49".to_string(),
            width: WIDTH,
            height: HEIGHT,
            vsync: true,
            ..Default::default()
        })
        .run();
}
