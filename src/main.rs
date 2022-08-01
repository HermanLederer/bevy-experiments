mod plugins;

use bevy::{prelude::*, window::PresentMode};
use plugins::lesson_2::Lesson2Plugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(WindowDescriptor {
            present_mode: PresentMode::AutoNoVsync,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(Lesson2Plugin)
        .run();
}
