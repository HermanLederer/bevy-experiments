mod plugins;

use bevy::prelude::*;
use plugins::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(rects::RectsPlugin)
        .run();
}
