mod render;

use bevy::{
    prelude::*,
    reflect::{TypeUuid}, render::{RenderStage, render_resource::SpecializedRenderPipelines, RenderApp, render_phase::AddRenderCommand}, core_pipeline::core_2d::Transparent2d,
};

use self::render::{SpriteMeta, SimpleMesh2dPipeline, ExtractedSprites, DrawSprite, queue_sprites};

pub use self::render::SimpleMesh2d;

pub const SPRITE_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 3763343953151597128);

#[derive(Default)]
pub struct SimpleMesh2dPlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum SpriteSystem {
    ExtractSprites,
}

impl Plugin for SimpleMesh2dPlugin {
    fn build(&self, app: &mut App) {
        let mut shaders = app.world.resource_mut::<Assets<Shader>>();
        let sprite_shader = Shader::from_wgsl(include_str!("render/sprite.wgsl"));
        shaders.set_untracked(SPRITE_SHADER_HANDLE, sprite_shader);
        app.add_asset::<TextureAtlas>();

        if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
            render_app
                .init_resource::<SimpleMesh2dPipeline>()
                .init_resource::<SpecializedRenderPipelines<SimpleMesh2dPipeline>>()
                .init_resource::<SpriteMeta>()
                .init_resource::<ExtractedSprites>()
                .add_render_command::<Transparent2d, DrawSprite>()
                .add_system_to_stage(
                    RenderStage::Extract,
                    render::extract_sprites.label(SpriteSystem::ExtractSprites),
                )
                .add_system_to_stage(RenderStage::Queue, queue_sprites);
        };

        app.add_system(rainbow_system);
    }
}

fn rainbow_system(time: Res<Time>, mut query: Query<&mut SimpleMesh2d>) {
    for mut spr in query.iter_mut() {
        spr.t += time.delta().as_secs_f32();
    }
}
