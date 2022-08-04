use std::f32::consts::PI;

use bevy::{prelude::*, render::camera::Projection};

//
//
// Plugin

pub struct Lesson3Plugin;

impl Plugin for Lesson3Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(init_system)
            .add_system(float_and_rotate);
    }
}

//
//
// Components

#[derive(Component)]
struct FloaterRotator();

//
//
// Resources

//
//
// Systems

fn init_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Camera
    commands.spawn_bundle(Camera3dBundle {
        projection: Projection::Perspective(PerspectiveProjection {
            fov: 76.0,
            ..default()
        }),
        ..default()
    });

    commands
        .spawn_bundle(PbrBundle {
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, -6.0),
                ..default()
            },
            mesh: meshes.add(Mesh::from(shape::Box::new(1.0, 1.0, 1.0))),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                metallic: 0.0,
                perceptual_roughness: 0.8,
                ..default()
            }),
            ..default()
        })
        .insert(FloaterRotator());

    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 4000.0,
            color: Color::WHITE,
            ..default()
        },
        transform: Transform {
            rotation: Quat::from_rotation_y(PI * 0.25),
            ..default()
        },
        ..default()
    });

    commands.insert_resource(ClearColor(Color::WHITE * 0.1));

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 0.1,
    });
}

fn float_and_rotate(time: Res<Time>, mut q: Query<(&mut Transform, &FloaterRotator)>) {
    q.for_each_mut(|(mut trns, _)| {
        trns.translation.y = time.time_since_startup().as_secs_f32().sin() * 0.2;
        trns.rotate_y(time.delta().as_secs_f32());
    });
}
