pub mod perf_log;
pub mod radial_physics;
pub mod rainbow_material;
// pub mod shapes;

use rand::Rng;
use std::f32::consts::PI;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use self::{radial_physics::RadialPhysicsPlugin, rainbow_material::RainbowMaterialPlugin, perf_log::PerfLogPlugin};
use crate::plugins::lesson_2::{
    radial_physics::{CircleCollider, Force},
    rainbow_material::RainbowMaterial,
};

//
//
// Plugin

pub struct Lesson2Plugin;

impl Plugin for Lesson2Plugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(PerfLogPlugin)
            .add_plugin(RainbowMaterialPlugin)
            .add_plugin(RadialPhysicsPlugin)
            .insert_resource(NextSpawnTime(0.0))
            .add_startup_system(init_system)
            .add_system(sprite_color_system)
            .add_system(input_system)
            .add_system(kill_system);
    }
}

//
//
// Components

#[derive(Component)]
pub struct Offset(f32);

#[derive(Component)]
pub struct Health(f32);

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

fn sprite_color_system(time: Res<Time>, mut query: Query<(&mut Sprite, &Offset)>) {
    let t = time.seconds_since_startup() as f32 * 3.0;
    for (mut spr, offst) in query.iter_mut() {
        spr.color = Color::rgb(
            (t + offst.0).sin().abs() as f32,
            (t + offst.0 + (PI * 0.33333)).sin().abs(),
            (t as f32 + offst.0 + (PI * 0.66666)).sin().abs(),
        );
    }
}

fn input_system(
    t: Res<Time>,
    mut next_t: ResMut<NextSpawnTime>,
    windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<RainbowMaterial>>,
    mut commands: Commands,
) {
    const DELAY: f64 = 0.01;
    let mut rng = rand::thread_rng();

    if t.seconds_since_startup() >= next_t.0 {
        let window = windows.get_primary().unwrap();

        if buttons.pressed(MouseButton::Left) {
            if let Some(pos) = window.cursor_position() {
                next_t.0 = t.seconds_since_startup() + DELAY;
                let size: f32 = rng.gen_range(4.0..=32.0);
                // let win_size = f32::min(window.width(), window.height());
                // let size: f32 = rng.gen_range((win_size * 0.25)..=(win_size * 0.5));

                let pos = Vec3::new(
                    pos.x - window.width() * 0.5,
                    pos.y - window.height() * 0.5,
                    0.0,
                );

                commands
                    .spawn_bundle(MaterialMesh2dBundle {
                        transform: Transform {
                            translation: pos,
                            scale: Vec3::splat(size),
                            ..default()
                        },
                        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                        // material: materials.add(ColorMaterial::from(Color::hsl(
                        //     rng.gen::<f32>() * 360.0,
                        //     1.0,
                        //     0.75,
                        // ))),
                        // material: materials.add(RainbowMaterial::default()),
                        material: materials.add(RainbowMaterial {
                            t: rng.gen::<f32>() * PI,
                        }),
                        // material: materials.add(RainbowMaterial { t: t.time_since_startup().as_secs_f32() }),
                        ..default()
                    })
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
                    .insert(Force {
                        velo: Vec3::new(
                            rng.gen_range(-200.0..=200.0),
                            rng.gen_range(-200.0..=200.0),
                            rng.gen_range(-200.0..=200.0),
                        ),
                    })
                    .insert(CircleCollider { r: 0.5 })
                    .insert(Health(size))
                    .insert(Offset(rng.gen::<f32>() * PI));
            } else {
                // cursor is not inside the window
            }
        }
    }
}

fn kill_system(
    t: Res<Time>,
    buttons: Res<Input<KeyCode>>,
    mut query: Query<(Entity, &mut Transform, &mut Health)>,
    mut commands: Commands,
) {
    if buttons.pressed(KeyCode::Space) {
        query.for_each_mut(|(ntt, mut trns, mut health)| {
            if health.0 <= 0.0 {
                commands.entity(ntt).despawn();
            } else {
                trns.scale = Vec3::splat(health.0);
                health.0 -= t.delta().as_secs_f32() * 32.0;
            }
        });
    }
}
