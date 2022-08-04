mod plugins;

use bevy::{prelude::*, window::PresentMode};
use plugins::lesson_3::Lesson3Plugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(WindowDescriptor {
            present_mode: PresentMode::AutoNoVsync,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(Lesson3Plugin)
        .run();
}
