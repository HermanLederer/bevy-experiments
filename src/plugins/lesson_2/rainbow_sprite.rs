use std::f32::consts::PI;

use bevy::prelude::*;

//
//
// Plugin

pub struct RainbowSpritePlugin;

impl Plugin for RainbowSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(sprite_color_system);
    }
}

//
//
// Components

#[derive(Component)]
pub struct Offset {
    pub delta: f32,
}

//
//
// Resources

//
//
// Systems

fn sprite_color_system(time: Res<Time>, mut query: Query<(&mut Sprite, &Offset)>) {
    let t = time.seconds_since_startup() as f32 * 3.0;
    for (mut spr, offst) in query.iter_mut() {
        spr.color = Color::rgb(
            (t + offst.delta).sin().abs() as f32,
            (t + offst.delta + (PI * 0.33333)).sin().abs(),
            (t as f32 + offst.delta + (PI * 0.66666)).sin().abs(),
        );
    }
}
