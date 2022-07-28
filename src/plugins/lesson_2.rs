pub mod custom_mesh;
pub mod shapes;

use std::f32::consts::PI;

use bevy::{prelude::*, sprite::Mesh2dHandle, utils::HashMap};
use rand::{prelude::ThreadRng, Rng};

use self::{
    custom_mesh::{ColoredMesh2d, ColoredMesh2dPlugin},
    shapes::*,
};

//
//
// Helper functions

fn rand_range(rng: &mut ThreadRng, min: f32, max: f32) -> f32 {
    rng.gen::<f32>() * (max - min) + max
}

fn rand_vec3(rng: &mut ThreadRng, range: &f32) -> Vec3 {
    let halfrange: f32 = range * 0.5;
    Vec3::new(
        rng.gen::<f32>() * range - halfrange,
        rng.gen::<f32>() * range - halfrange,
        rng.gen::<f32>() * range - halfrange,
    )
}

//
//
// Plugin

pub struct Lesson2Plugin;

impl Plugin for Lesson2Plugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(NextSpawnTime(0.0))
            .add_plugin(ColoredMesh2dPlugin)
            .add_startup_system(init_system)
            .add_system(sprite_color_system)
            .add_system(movement_system)
            .add_system(input_system)
            .add_system(kill_system);
    }
}

//
//
// Components

#[derive(Component)]
pub struct Offset(f32);

#[derive(Component, Clone, Copy)]
pub struct Force {
    pub velo: Vec3,
}

#[derive(Component, Clone, Copy)]
pub struct CircleCollider(f32);

#[derive(Component)]
pub struct Health {
    max: f32,
    current: f32,
}

//
//
// Resources

#[derive(Default)]
struct NextSpawnTime(f64);

struct RenderResources {
    mesh: Handle<Mesh>,
}

//
//
// Systems

fn spawn_rect(
    commands: &mut Commands,
    meshes: &Res<Assets<Mesh>>,
    render_res: &Res<RenderResources>,
    pos: Vec3,
) {
    let mut rng = rand::thread_rng();

    let size = rand_range(&mut rng, 1.0, 16.0);

    // We can now spawn the entities for the star and the camera
    commands
        .spawn_bundle((
            // We use a marker component to identify the custom colored meshes
            ColoredMesh2d::default(),
            // The `Handle<Mesh>` needs to be wrapped in a `Mesh2dHandle` to use 2d rendering instead of 3d
            Mesh2dHandle(meshes.get_handle(&render_res.mesh).into()),
            // These other components are needed for 2d meshes to be rendered
            Transform {
                translation: pos,
                scale: Vec3::splat(size),
                ..default()
            },
            GlobalTransform::default(),
            Visibility::default(),
            ComputedVisibility::default(),
        ))
        // .spawn_bundle(SpriteBundle {
        //     sprite: Sprite {
        //         color: Color::WHITE,
        //         ..default()
        //     },
        //     transform: Transform {
        //         translation: pos,
        //         ..default()
        //     },
        //     ..default()
        // })
        .insert(Force {
            velo: rand_vec3(&mut rng, &200.0),
        })
        .insert(CircleCollider(0.5))
        .insert(Health {
            max: size,
            current: size,
        })
        .insert(Offset(rng.gen::<f32>() * PI));
}

fn init_system(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    // Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // Render assets
    commands.insert_resource(RenderResources {
        mesh: meshes.add(create_circle(10)),
        // mesh: meshes.add(create_star(0.25, 0.25)),
    });
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

fn movement_system(
    windows: Res<Windows>,
    time: Res<Time>,
    // res_opt: Option<Res<(Time, Bounds)>>,
    mut query: Query<(Entity, &mut Transform, &CircleCollider, &mut Force)>,
) {
    // Copy entities to a hash map
    let mut entities: HashMap<u32, (Vec3, Vec3)> = HashMap::new();
    query.for_each(|(ntt, trns, _, frc)| {
        entities.insert(ntt.id(), (trns.translation.clone(), frc.velo.clone()));
    });

    // Update entities in hash map
    for (ntt, trns, col, _) in query.iter() {
        let (mut pos, mut velo) = entities.get(&ntt.id()).unwrap();

        let r = col.0 * trns.scale.x;

        // Move
        pos += velo * time.delta().as_secs_f32();
        pos = Vec3::new(pos.x, pos.y, 0.0);

        let win = windows.get_primary().unwrap();
        let left = win.width() * -0.5;
        let right = win.width() * 0.5;
        let bottom = win.height() * -0.5;
        let top = win.height() * 0.5;

        // Collide with others
        for (ntt_other, trns_other, col_other, _) in query.iter() {
            if ntt_other == ntt {
                // Do not collide with self
                break;
            }

            let (mut pos_other, mut velo_other) = entities.get(&ntt_other.id()).unwrap();
            let r_other = col_other.0 * trns_other.scale.x;

            let dist = Vec3::distance(pos, pos_other);
            let r_sum = r + r_other;

            if dist <= r_sum {
                let towards_self;
                let towards_other;

                if dist == 0.0 {
                    let mut rng = rand::thread_rng();

                    towards_self = Vec3::new(rng.gen(), rng.gen(), rng.gen()).normalize();
                    towards_other = Vec3::new(rng.gen(), rng.gen(), rng.gen()).normalize();
                } else {
                    towards_self = (pos_other - pos).normalize();
                    towards_other = -towards_self;
                }

                let temp = velo;
                // velo = towards_other * velo.length();
                // velo_other = towards_self * velo_other.length();
                velo = velo_other;
                velo_other = temp;

                pos += towards_self * (dist - r_sum);
                pos_other += towards_other * (dist - r_sum);

                entities.insert(ntt.id(), (pos, velo));
                entities.insert(ntt_other.id(), (pos_other, velo_other));
            }
        }

        // Collide with bounds

        if pos.x - r <= left {
            velo = Vec3::new(-velo.x, velo.y, 0.0);
            let inset = Vec3::new(pos.x - r - left, 0.0, 0.0);
            pos -= inset;
        }

        if pos.x + r >= right {
            velo = Vec3::new(-velo.x, velo.y, 0.0);
            let inset = Vec3::new(pos.x + r - right, 0.0, 0.0);
            pos -= inset;
        }

        if pos.y - r <= bottom {
            velo = Vec3::new(velo.x, -velo.y, 0.0);
            let inset = Vec3::new(0.0, pos.y - r - bottom, 0.0);
            pos -= inset;
        }

        if pos.y + r >= top {
            velo = Vec3::new(velo.x, -velo.y, 0.0);
            let inset = Vec3::new(0.0, pos.y + r - top, 0.0);
            pos -= inset;
        }

        entities.insert(ntt.id(), (pos, velo));
    }

    // Write updates to entities
    for (ntt, mut trns, _, mut frc) in query.iter_mut() {
        let (new_pos, new_velo) = entities.get(&ntt.id()).unwrap();
        trns.translation = new_pos.clone();
        frc.velo = new_velo.clone();
    }
}

fn input_system(
    t: Res<Time>,
    mut next_t: ResMut<NextSpawnTime>,
    windows: Res<Windows>,
    buttons: Res<Input<MouseButton>>,
    meshes: Res<Assets<Mesh>>,
    render_res: Res<RenderResources>,
    mut commands: Commands,
) {
    const DELAY: f64 = 0.01;

    if t.seconds_since_startup() >= next_t.0 {
        let window = windows.get_primary().unwrap();

        if buttons.pressed(MouseButton::Left) {
            if let Some(pos) = window.cursor_position() {
                next_t.0 = t.seconds_since_startup() + DELAY;
                spawn_rect(
                    &mut commands,
                    &meshes,
                    &render_res,
                    Vec3::new(
                        pos.x - window.width() * 0.5,
                        pos.y - window.height() * 0.5,
                        0.0,
                    ),
                );
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
            if health.current <= 0.0 {
                commands.entity(ntt).despawn();
            } else {
                trns.scale = Vec3::splat(health.current);
                health.current -= t.delta().as_secs_f32() * 32.0;
            }
        });
    }
}
