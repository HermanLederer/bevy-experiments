pub mod bevy_radial_physics;
pub mod fast_rainbow_material;
pub mod perf_log;
// pub mod rainbow_material;
// pub mod rainbow_sprite;
// pub mod shapes;
pub mod size_and_lifetime;

use rand::Rng;
use std::f32::consts::PI;

use bevy::prelude::*;

use self::{
    bevy_radial_physics::{CircleCollider, Force, RadialPhysicsPlugin},
    fast_rainbow_material::{SimpleMesh2d, SimpleMesh2dPlugin},
    perf_log::PerfLogPlugin,
    size_and_lifetime::{Health, SizeAndLifetimePlugin},
};
//
//
// Plugin

pub struct Lesson2Plugin;

impl Plugin for Lesson2Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PerfLogPlugin)
            .add_plugin(RadialPhysicsPlugin)
            .add_plugin(SizeAndLifetimePlugin)
            .add_plugin(SimpleMesh2dPlugin)
            .insert_resource(NextSpawnTime(0.0))
            .add_startup_system(init_system)
            // .add_startup_system(hot_start_system)
            .add_system(input_system);
    }
}

//
//
// Components

//
//
// Resources

#[derive(Default)]
struct NextSpawnTime(f64);

//
//
// Systems

fn init_system(mut commands: Commands) {
    // Camera
    commands.spawn_bundle(Camera2dBundle::default());
}

fn hot_start_system(mut commands: Commands) {
    for _ in 0..1024 {
        spawn_random_dot(&mut commands);
    }
}

fn input_system(
    t: Res<Time>,
    mut next_t: ResMut<NextSpawnTime>,
    windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
    mut commands: Commands,
) {
    const DELAY: f64 = 0.01;

    if t.seconds_since_startup() >= next_t.0 {
        let window = windows.get_primary().unwrap();

        if buttons.pressed(MouseButton::Left) {
            if let Some(pos) = window.cursor_position() {
                next_t.0 = t.seconds_since_startup() + DELAY;

                let pos = Vec3::new(
                    pos.x - window.width() * 0.5,
                    pos.y - window.height() * 0.5,
                    0.0,
                );

                spawn_random_dot_at(&mut commands, pos);
            } else {
                // cursor is not inside the window
            }
        }
    }
}

//
//
// Helpers

fn spawn_dot(commands: &mut Commands, pos: Vec3, size: f32, velo: Vec3, color_offset: f32) {
    commands
        .spawn_bundle((
            SimpleMesh2d { t: color_offset },
            Transform {
                translation: pos,
                scale: Vec3::splat(size),
                ..default()
            },
            GlobalTransform::default(),
            Visibility::default(),
            ComputedVisibility::default(),
        ))
        .insert(Force { velo })
        .insert(CircleCollider { r: 0.5 })
        .insert(Health { value: size });
}

fn spawn_random_dot_at(mut commands: &mut Commands, pos: Vec3) {
    let mut rng = rand::thread_rng();

    let size: f32 = rng.gen_range(4.0..=32.0);
    // let win_size = f32::min(window.width(), window.height());
    // let size: f32 = rng.gen_range((win_size * 0.25)..=(win_size * 0.5));

    let velo = Vec3::new(
        rng.gen_range(-200.0..=200.0),
        rng.gen_range(-200.0..=200.0),
        rng.gen_range(-200.0..=200.0),
    );

    let color_offset = rng.gen::<f32>() * PI;

    spawn_dot(&mut commands, pos, size, velo, color_offset);
}

fn spawn_random_dot(mut commands: &mut Commands) {
    spawn_random_dot_at(&mut commands, Vec3::ZERO);
}
