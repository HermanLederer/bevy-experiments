pub mod fast_rainbow_material;
pub mod perf_log;
pub mod radial_physics;
pub mod rainbow_material;
pub mod rainbow_sprite;
// pub mod shapes;
pub mod size_and_lifetime;

use rand::Rng;
use std::f32::consts::PI;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle, render::texture::DEFAULT_IMAGE_HANDLE};

use self::{
    fast_rainbow_material::{SimpleMesh2d, SimpleMesh2dPlugin},
    perf_log::PerfLogPlugin,
    radial_physics::RadialPhysicsPlugin,
    rainbow_material::RainbowMaterialPlugin,
    rainbow_sprite::{Offset, RainbowSpritePlugin},
    size_and_lifetime::{Health, SizeAndLifetimePlugin},
};
use crate::plugins::lesson_2::{
    radial_physics::{CircleCollider, Force},
    // rainbow_material::RainbowMaterial,
};

//
//
// Plugin

pub struct Lesson2Plugin;

impl Plugin for Lesson2Plugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PerfLogPlugin)
            // .add_plugin(RainbowMaterialPlugin)
            .add_plugin(RadialPhysicsPlugin)
            .add_plugin(SizeAndLifetimePlugin)
            .add_plugin(SimpleMesh2dPlugin)
            // .add_plugin(RainbowSpritePlugin)
            .insert_resource(NextSpawnTime(0.0))
            .add_startup_system(init_system)
            .add_startup_system(hot_start_system)
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

fn hot_start_system(
    mut commands: Commands,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<RainbowMaterial>>,
) {
    for _ in 0..1024 {
        spawn_random_dot(
            &mut commands,
            // &mut meshes,
            // &mut materials
        );
    }
}

fn input_system(
    t: Res<Time>,
    mut next_t: ResMut<NextSpawnTime>,
    windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
    // mut meshes: ResMut<Assets<Mesh>>,
    // mut materials: ResMut<Assets<RainbowMaterial>>,
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

                spawn_random_dot_at(
                    &mut commands,
                    // &mut meshes,
                    // &mut materials,
                    pos
                );
            } else {
                // cursor is not inside the window
            }
        }
    }
}

//
//
// Helpers

fn spawn_dot(
    commands: &mut Commands,
    // meshes: &mut ResMut<Assets<Mesh>>,
    // materials: &mut ResMut<Assets<RainbowMaterial>>,
    pos: Vec3,
    size: f32,
    velo: Vec3,
    color_offset: f32,
) {
    commands
        .spawn_bundle((
            // We use a marker component to identify the custom colored meshes
            SimpleMesh2d::default(),
            // These other components are needed for 2d meshes to be rendered
            Transform {
                translation: pos,
                scale: Vec3::splat(size),
                ..default()
            },
            GlobalTransform::default(),
            DEFAULT_IMAGE_HANDLE.typed::<Image>(),
            Visibility::default(),
            ComputedVisibility::default(),
        ))
        // .spawn_bundle(MaterialMesh2dBundle {
        //     transform: Transform {
        //         translation: pos,
        //         scale: Vec3::splat(size),
        //         ..default()
        //     },
        //     mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        //     // material: materials.add(ColorMaterial::from(Color::hsl(
        //     //     rng.gen::<f32>() * 360.0,
        //     //     1.0,
        //     //     0.75,
        //     // ))),
        //     // material: materials.add(RainbowMaterial::default()),
        //     material: materials.add(RainbowMaterial { t: color_offset }),
        //     // material: materials.add(RainbowMaterial { t: t.time_since_startup().as_secs_f32() }),
        //     ..default()
        // })
        // .spawn_bundle(SpriteBundle {
        //     sprite: Sprite {
        //         color: Color::WHITE,
        //         ..default()
        //     },
        //     transform: Transform {
        //         translation: pos,
        //         scale: Vec3::splat(size),
        //         ..default()
        //     },
        //     ..default()
        // })
        // .insert(Offset {
        //     delta: color_offset,
        // })
        .insert(Force { velo })
        .insert(CircleCollider { r: 0.5 })
        .insert(Health { value: size });
}

fn spawn_random_dot_at(
    mut commands: &mut Commands,
    // mut meshes: &mut ResMut<Assets<Mesh>>,
    // mut materials: &mut ResMut<Assets<RainbowMaterial>>,
    pos: Vec3,
) {
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

    spawn_dot(
        &mut commands,
        // &mut meshes,
        // &mut materials,
        pos,
        size,
        velo,
        color_offset,
    );
}

fn spawn_random_dot(
    mut commands: &mut Commands,
    // mut meshes: &mut ResMut<Assets<Mesh>>,
    // mut materials: &mut ResMut<Assets<RainbowMaterial>>,
) {
    spawn_random_dot_at(
        &mut commands,
        // &mut meshes,
        // &mut materials,
        Vec3::ZERO
    );
}
