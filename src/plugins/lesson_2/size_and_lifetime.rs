use bevy::prelude::*;

//
//
// Plugin

pub struct SizeAndLifetimePlugin;

impl Plugin for SizeAndLifetimePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(lifetime_system);
    }
}

//
//
// Components

#[derive(Component)]
pub struct Health {
    pub value: f32
}

//
//
// Resources

//
//
// Systems

fn lifetime_system(
    t: Res<Time>,
    buttons: Res<Input<KeyCode>>,
    mut query: Query<(Entity, &mut Transform, &mut Health)>,
    mut commands: Commands,
) {
    if buttons.pressed(KeyCode::Space) {
        query.for_each_mut(|(ntt, mut trns, mut health)| {
            if health.value <= 0.0 {
                commands.entity(ntt).despawn();
            } else {
                trns.scale = Vec3::splat(health.value);
                health.value -= t.delta().as_secs_f32() * 32.0;
            }
        });
    }
}
