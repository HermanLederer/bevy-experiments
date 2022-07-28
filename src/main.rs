mod plugins;

use bevy::prelude::*;
use plugins::lesson_2::Lesson2Plugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(Lesson2Plugin)
        .run();
}
