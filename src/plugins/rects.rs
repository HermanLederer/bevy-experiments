use std::f32::consts::PI;

use bevy::{ecs::event::Events, prelude::*, utils::HashMap, window::WindowResized};
use rand::{prelude::ThreadRng, Rng};

pub struct RectsPlugin;

impl Plugin for RectsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Bounds {
            width: 0.0,
            height: 0.0,
        })
        .insert_resource(NextSpawnTime(0.0))
        .add_startup_system(init_system)
        .add_system(sprite_color_system)
        .add_system(movement_system)
        .add_system(input_system)
        .add_system(kill_system)
        .add_system(resize_notificator);
    }
}

#[derive(Component)]
pub struct Offset(f32);

#[derive(Component, Clone, Copy)]
pub struct Force {
    pub velo: Vec3,
}

#[derive(Component, Clone, Copy)]
pub struct CircleCollider(f32);

#[derive(Component)]
pub struct Health(f32);

#[derive(Default)]
struct NextSpawnTime(f64);

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

fn spawn_rect(commands: &mut Commands, pos: Vec3) {
    let mut rng = rand::thread_rng();

    let s = rand_range(&mut rng, 1.0, 6.0);
    let size = Vec2::new(s, s);

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(size),
                ..default()
            },
            transform: Transform {
                translation: pos,
                ..default()
            },
            ..default()
        })
        .insert(Force {
            velo: rand_vec3(&mut rng, &200.0),
        })
        .insert(CircleCollider(s))
        .insert(Health(1.0))
        .insert(Offset(rng.gen::<f32>() * PI));
}

fn init_system(mut commands: Commands) {
    // Camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
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
    bnds: Res<Bounds>,
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
    for (ntt, _, col, _) in query.iter() {
        let (mut pos, mut velo) = entities.get(&ntt.id()).unwrap();

        let r = col.0;

        // Move
        pos += velo * time.delta().as_secs_f32();
        pos = Vec3::new(pos.x, pos.y, 0.0);

        let left = bnds.width * -0.5;
        let right = bnds.width * 0.5;
        let bottom = bnds.height * -0.5;
        let top = bnds.height * 0.5;

        // Collide with others
        for (ntt_other, _, col_other, _) in query.iter() {
            if ntt_other == ntt {
                // Do not collide with self
                break;
            }

            let (mut pos_other, mut velo_other) = entities.get(&ntt_other.id()).unwrap();
            let r_other = col_other.0;

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

                velo = towards_other * velo.length();
                velo_other = towards_self * velo_other.length();

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
    bnds: Res<Bounds>,
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
                spawn_rect(
                    &mut commands,
                    Vec3::new(pos.x - bnds.width * 0.5, pos.y - bnds.height * 0.5, 0.0),
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
            if health.0 <= 0.0 {
                commands.entity(ntt).despawn();
            } else {
                let scale = health.0;
                trns.scale = Vec3::new(scale, scale, scale);
                health.0 -= t.delta().as_secs_f32() * 2.0;
            }
        });
    }
}

#[derive(Default)]
struct Bounds {
    width: f32,
    height: f32,
}

fn resize_notificator(mut bnds: ResMut<Bounds>, resize_event: Res<Events<WindowResized>>) {
    let mut reader = resize_event.get_reader();
    for e in reader.iter(&resize_event) {
        (bnds.width, bnds.height) = (e.width, e.height);
    }
}
