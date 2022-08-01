use rand::Rng;

use bevy::{prelude::*, utils::HashMap};

//
//
// Plugin

pub struct RadialPhysicsPlugin;

impl Plugin for RadialPhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(movement_system);
    }
}

//
//
// Components

#[derive(Component, Clone, Copy)]
pub struct Force {
    pub velo: Vec3,
}

#[derive(Component, Clone, Copy)]
pub struct CircleCollider {
    pub r: f32,
}

//
//
// Resources

//
//
// Systems

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

        let r = col.r * trns.scale.x;

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
            let r_other = col_other.r * trns_other.scale.x;

            let dist = Vec3::distance(pos, pos_other);
            let r_sum = r + r_other;

            if dist <= r_sum {
                let towards_self;
                let towards_other;

                if dist == 0.0 {
                    // Colliders are completely clipped into each other,
                    // their positions are the same,
                    // there is no direction away from each other
                    // so we generate a random one
                    let mut rng = rand::thread_rng();

                    towards_self = Vec3::new(rng.gen(), rng.gen(), rng.gen()).normalize();
                    towards_other = Vec3::new(rng.gen(), rng.gen(), rng.gen()).normalize();
                } else {
                    towards_self = (pos_other - pos).normalize();
                    towards_other = -towards_self;
                }

                let temp = velo;
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
