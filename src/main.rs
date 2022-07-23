use bevy::prelude::*;

pub struct TestPlugin;

impl Plugin for TestPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_test_constructions)
            .add_system(test_system);
    }
}

#[derive(Component)]
struct Construction;

#[derive(Component)]
struct Name(String);

fn test_system(query: Query<&Name, With<Construction>>) {
    for name in query.iter() {
        println!("You see {}!", name.0);
    }
}

fn spawn_test_constructions(mut commands: Commands) {
    for n in 0..12 {
        commands.spawn()
            .insert(Construction)
            .insert(Name(("Construction ".to_owned() + &n.to_string()).to_string()));
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TestPlugin)
        .run();
}
