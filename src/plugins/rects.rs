use std::f32::consts::PI;

use bevy::prelude::*;
use rand::Rng;

pub struct RectsPlugin;

impl Plugin for RectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_rects)
            .add_system(movement_system);
    }
}

#[derive(Component)]
pub struct Offset(f32);

fn spawn_rects(mut commands: Commands) {
    let mut rng = rand::thread_rng();

    // Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Rects
    let range: f32 = 200.0;
    let halfrange: f32 = range * 0.5;

    for _ in 0..64 {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.0, 0.0, 0.0),
                    custom_size: Some(Vec2::new(50.0, 50.0)),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3::new(
                        rng.gen::<f32>() * range - halfrange,
                        rng.gen::<f32>() * range - halfrange,
                        rng.gen::<f32>() * range - halfrange,
                    ),
                    ..default()
                },
                ..default()
            })
            .insert(Offset(rng.gen::<f32>() * PI));
    }
}

fn movement_system(time: Res<Time>, mut query: Query<(&mut Sprite, &Offset)>) {
    for (mut spr, offst) in query.iter_mut() {
        spr.color = Color::rgb(
            (time.seconds_since_startup() as f32 + offst.0).sin().abs() as f32,
            (time.seconds_since_startup() as f32 + offst.0 + (PI * 0.33333))
                .sin()
                .abs(),
            (time.seconds_since_startup() as f32 + offst.0 + (PI * 0.66666))
                .sin()
                .abs(),
        );
    }
}
