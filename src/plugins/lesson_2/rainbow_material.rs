use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::*,
    sprite::{Material2d, Material2dPlugin},
};

//
//
// Plugin

#[derive(Default)]
pub struct RainbowMaterialPlugin;

impl Plugin for RainbowMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(Material2dPlugin::<RainbowMaterial>::default());
        app.world
            .resource_mut::<Assets<RainbowMaterial>>()
            .set_untracked(
                Handle::<RainbowMaterial>::default(),
                RainbowMaterial {
                    color: Color::rgb(1.0, 0.0, 1.0),
                    ..Default::default()
                },
            );
    }
}

//
//
// Components

//
//
// Resources

//
//
// Materials

#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "a8dc184b-fb25-4bb8-a1e2-c364d73b5183"]
pub struct RainbowMaterial {
    #[uniform(0)]
    pub color: Color
}

impl Material2d for RainbowMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/rainbow.wgsl".into()
    }
}

impl Default for RainbowMaterial {
    fn default() -> Self {
        RainbowMaterial {
            color: Color::WHITE,
        }
    }
}

//
//
// Systems
