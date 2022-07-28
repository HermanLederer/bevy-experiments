mod plugins;

use bevy::{
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
    sprite::Mesh2dHandle,
};
use plugins::lesson_2::{
    custom_mesh::{ColoredMesh2d, ColoredMesh2dPlugin},
    Lesson2Plugin,
};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins)
        .add_plugin(Lesson2Plugin)
        .run();
}
