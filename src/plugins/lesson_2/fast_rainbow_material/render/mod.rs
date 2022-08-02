use bevy::{
    core_pipeline::core_2d::Transparent2d,
    ecs::{
        prelude::*,
        system::{lifetimeless::*, SystemParamItem},
    },
    prelude::*,
    render::{
        render_phase::{
            BatchedPhaseItem, DrawFunctions, EntityRenderCommand, RenderCommand,
            RenderCommandResult, RenderPhase, SetItemPipeline, TrackedRenderPass,
        },
        render_resource::*,
        renderer::{RenderDevice, RenderQueue},
        texture::BevyDefault,
        view::{
            ComputedVisibility, Msaa, ViewUniform, ViewUniformOffset, ViewUniforms, VisibleEntities,
        },
        Extract,
    },
    utils::FloatOrd,
};
use bytemuck::{Pod, Zeroable};
use copyless::VecHelper;
use fixedbitset::FixedBitSet;

use super::SPRITE_SHADER_HANDLE;

/// A marker component
#[derive(Component, Default)]
pub struct SimpleMesh2d {
    pub t: f32,
}

/// Custom pipeline
pub struct SimpleMesh2dPipeline {
    view_layout: BindGroupLayout,
    // material_layout: BindGroupLayout,
}

impl FromWorld for SimpleMesh2dPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        let view_layout = render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX | ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: true,
                    min_binding_size: Some(ViewUniform::min_size()),
                },
                count: None,
            }],
            label: Some("sprite_view_layout"),
        });

        // let material_layout = render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
        //     entries: &[BindGroupLayoutEntry {
        //         binding: 0,
        //         visibility: ShaderStages::FRAGMENT,
        //         ty: BindingType::Buffer {
        //             ty: BufferBindingType::Uniform,
        //             has_dynamic_offset: true,
        //             min_binding_size: Some(ViewUniform::min_size()),
        //         },
        //         count: None,
        //     }],
        //     label: Some("sprite_material_layout"),
        // });

        SimpleMesh2dPipeline {
            view_layout,
            // material_layout,
        }
    }
}

bitflags::bitflags! {
    #[repr(transparent)]
    // NOTE: Apparently quadro drivers support up to 64x MSAA.
    // MSAA uses the highest 6 bits for the MSAA sample count - 1 to support up to 64x MSAA.
    pub struct SimpleMesh2dPipelineKey: u32 {
        const NONE                        = 0;
        const COLORED                     = (1 << 0);
        const MSAA_RESERVED_BITS          = SimpleMesh2dPipelineKey::MSAA_MASK_BITS << SimpleMesh2dPipelineKey::MSAA_SHIFT_BITS;
    }
}

impl SimpleMesh2dPipelineKey {
    const MSAA_MASK_BITS: u32 = 0b111111;
    const MSAA_SHIFT_BITS: u32 = 32 - 6;

    pub fn from_msaa_samples(msaa_samples: u32) -> Self {
        let msaa_bits = ((msaa_samples - 1) & Self::MSAA_MASK_BITS) << Self::MSAA_SHIFT_BITS;
        SimpleMesh2dPipelineKey::from_bits(msaa_bits).unwrap()
    }

    pub fn msaa_samples(&self) -> u32 {
        ((self.bits >> Self::MSAA_SHIFT_BITS) & Self::MSAA_MASK_BITS) + 1
    }
}

impl SpecializedRenderPipeline for SimpleMesh2dPipeline {
    type Key = SimpleMesh2dPipelineKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        let mut formats = vec![
            // position
            VertexFormat::Float32x3,
            // uv
            VertexFormat::Float32x2,
            // t
            VertexFormat::Float32,
        ];

        if key.contains(SimpleMesh2dPipelineKey::COLORED) {
            // color
            formats.push(VertexFormat::Float32x4);
        }

        let vertex_layout =
            VertexBufferLayout::from_vertex_formats(VertexStepMode::Vertex, formats);

        let mut shader_defs = Vec::new();
        if key.contains(SimpleMesh2dPipelineKey::COLORED) {
            shader_defs.push("COLORED".to_string());
        }

        RenderPipelineDescriptor {
            vertex: VertexState {
                shader: SPRITE_SHADER_HANDLE.typed::<Shader>(),
                entry_point: "vertex".into(),
                shader_defs: shader_defs.clone(),
                buffers: vec![vertex_layout],
            },
            fragment: Some(FragmentState {
                shader: SPRITE_SHADER_HANDLE.typed::<Shader>(),
                shader_defs,
                entry_point: "fragment".into(),
                targets: vec![Some(ColorTargetState {
                    format: TextureFormat::bevy_default(),
                    blend: Some(BlendState::ALPHA_BLENDING),
                    write_mask: ColorWrites::ALL,
                })],
            }),
            // layout: Some(vec![self.view_layout.clone(), self.material_layout.clone()]),
            layout: Some(vec![self.view_layout.clone()]),
            primitive: PrimitiveState {
                front_face: FrontFace::Ccw,
                cull_mode: None,
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: key.msaa_samples(),
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            label: Some("sprite_pipeline".into()),
        }
    }
}

#[derive(Component, Clone, Copy)]
pub struct ExtractedSprite {
    pub entity: Entity,
    pub transform: GlobalTransform,
    pub t: f32,
}

#[derive(Default)]
pub struct ExtractedSprites {
    pub sprites: Vec<ExtractedSprite>,
}

pub fn extract_sprites(
    mut extracted_sprites: ResMut<ExtractedSprites>,
    // texture_atlases: Extract<Res<Assets<TextureAtlas>>>,
    sprite_query: Extract<Query<(Entity, &ComputedVisibility, &SimpleMesh2d, &GlobalTransform)>>,
) {
    extracted_sprites.sprites.clear();
    for (entity, visibility, simple_mesh_2d, transform) in sprite_query.iter() {
        if !visibility.is_visible() {
            continue;
        }
        // PERF: we don't check in this function that the `Image` asset is ready, since it should be in most cases and hashing the handle is expensive
        extracted_sprites.sprites.alloc().init(ExtractedSprite {
            entity,
            transform: *transform,
            t: simple_mesh_2d.t,
        });
    }
}

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
struct SpriteVertex {
    pub position: [f32; 3],
    pub uv: [f32; 2],
    pub t: f32,
}

// #[repr(C)]
// #[derive(Copy, Clone, Pod, Zeroable)]
// struct ColoredSpriteVertex {
//     pub position: [f32; 3],
//     pub uv: [f32; 2],
//     pub color: [f32; 4],
// }

pub struct SpriteMeta {
    vertices: BufferVec<SpriteVertex>,
    // colored_vertices: BufferVec<ColoredSpriteVertex>,
    view_bind_group: Option<BindGroup>,
}

impl Default for SpriteMeta {
    fn default() -> Self {
        Self {
            vertices: BufferVec::new(BufferUsages::VERTEX),
            // colored_vertices: BufferVec::new(BufferUsages::VERTEX),
            view_bind_group: None,
        }
    }
}

const QUAD_INDICES: [usize; 6] = [0, 2, 3, 0, 1, 2];

const QUAD_VERTEX_POSITIONS: [Vec2; 4] = [
    Vec2::new(-0.5, -0.5),
    Vec2::new(0.5, -0.5),
    Vec2::new(0.5, 0.5),
    Vec2::new(-0.5, 0.5),
];

const QUAD_UVS: [Vec2; 4] = [
    Vec2::new(0., 1.),
    Vec2::new(1., 1.),
    Vec2::new(1., 0.),
    Vec2::new(0., 0.),
];

#[derive(Component, Eq, PartialEq, Copy, Clone)]
pub struct SpriteBatch {
    // image_handle_id: HandleId,
    // colored: bool,
}

#[derive(Clone, ShaderType)]
struct SimpleMaterialUniforms {
    t: f32,
}

#[allow(clippy::too_many_arguments)]
pub fn queue_sprites(
    mut commands: Commands,
    mut view_entities: Local<FixedBitSet>,
    draw_functions: Res<DrawFunctions<Transparent2d>>,
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    mut sprite_meta: ResMut<SpriteMeta>,
    view_uniforms: Res<ViewUniforms>,
    sprite_pipeline: Res<SimpleMesh2dPipeline>,
    mut pipelines: ResMut<SpecializedRenderPipelines<SimpleMesh2dPipeline>>,
    mut pipeline_cache: ResMut<PipelineCache>,
    msaa: Res<Msaa>,
    mut extracted_sprites: ResMut<ExtractedSprites>,
    mut views: Query<(&VisibleEntities, &mut RenderPhase<Transparent2d>)>,
) {
    if let Some(view_binding) = view_uniforms.uniforms.binding() {
        let sprite_meta = &mut sprite_meta;

        // Clear the vertex buffers
        sprite_meta.vertices.clear();

        sprite_meta.view_bind_group = Some(render_device.create_bind_group(&BindGroupDescriptor {
            entries: &[BindGroupEntry {
                binding: 0,
                resource: view_binding,
            }],
            label: Some("sprite_view_bind_group"),
            layout: &sprite_pipeline.view_layout,
        }));

        let draw_sprite_function = draw_functions.read().get_id::<DrawSprite>().unwrap();
        let key = SimpleMesh2dPipelineKey::from_msaa_samples(msaa.samples);
        let pipeline = pipelines.specialize(&mut pipeline_cache, &sprite_pipeline, key);

        // Vertex buffer indices
        let mut index = 0;

        let extracted_sprites = &mut extracted_sprites.sprites;

        for (visible_entities, mut transparent_phase) in &mut views {
            view_entities.clear();
            view_entities.extend(visible_entities.entities.iter().map(|e| e.id() as usize));
            transparent_phase.items.reserve(extracted_sprites.len());

            // Impossible starting values that will be replaced on the first iteration
            let mut current_batch = SpriteBatch {
                // image_handle_id: HandleId::Id(Uuid::nil(), u64::MAX),
                // colored: false,
            };
            let mut current_batch_entity = Entity::from_raw(u32::MAX);
            // let mut current_image_size = Vec2::ZERO;
            // Add a phase item for each sprite, and detect when succesive items can be batched.
            // Spawn an entity with a `SpriteBatch` component for each possible batch.
            // Compatible items share the same entity.
            // Batches are merged later (in `batch_phase_system()`), so that they can be interrupted
            // by any other phase item (and they can interrupt other items from batching).
            for extracted_sprite in extracted_sprites.iter() {
                if !view_entities.contains(extracted_sprite.entity.id() as usize) {
                    continue;
                }
                let new_batch = SpriteBatch {
                    // image_handle_id: extracted_sprite.image_handle_id,
                    // colored: extracted_sprite.color != Color::WHITE,
                };
                if new_batch != current_batch {
                    // Set-up a new possible batch
                    current_batch = new_batch;
                    // current_image_size = Vec2::new(gpu_image.size.x, gpu_image.size.y);
                    current_batch_entity = commands.spawn_bundle((current_batch,)).id();
                }

                // Apply size and global transform
                let positions = QUAD_VERTEX_POSITIONS.map(|quad_pos| {
                    extracted_sprite
                        .transform
                        .mul_vec3((quad_pos).extend(0.))
                        .into()
                });

                // These items will be sorted by depth with other phase items
                let sort_key = FloatOrd(extracted_sprite.transform.translation().z);

                // Store the vertex data and add the item to the render phase
                for i in QUAD_INDICES {
                    sprite_meta.vertices.push(SpriteVertex {
                        position: positions[i],
                        uv: QUAD_UVS[i].into(),
                        t: extracted_sprite.t,
                    });
                }
                let item_start = index;
                index += QUAD_INDICES.len() as u32;
                let item_end = index;

                transparent_phase.add(Transparent2d {
                    draw_function: draw_sprite_function,
                    pipeline,
                    entity: current_batch_entity,
                    sort_key,
                    batch_range: Some(item_start..item_end),
                });
            }
        }
        sprite_meta
            .vertices
            .write_buffer(&render_device, &render_queue);
        // sprite_meta
        //     .colored_vertices
        //     .write_buffer(&render_device, &render_queue);
    }
}

pub type DrawSprite = (
    SetItemPipeline,
    SetSpriteViewBindGroup<0>,
    // SetSpriteTimeBindGroup<1>,
    DrawSpriteBatch,
);

pub struct SetSpriteViewBindGroup<const I: usize>;
impl<const I: usize> EntityRenderCommand for SetSpriteViewBindGroup<I> {
    type Param = (SRes<SpriteMeta>, SQuery<Read<ViewUniformOffset>>);

    fn render<'w>(
        view: Entity,
        _item: Entity,
        (sprite_meta, view_query): SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let view_uniform = view_query.get(view).unwrap();
        pass.set_bind_group(
            I,
            sprite_meta.into_inner().view_bind_group.as_ref().unwrap(),
            &[view_uniform.offset],
        );
        RenderCommandResult::Success
    }
}

// pub struct SetSpriteTimeBindGroup<const I: usize>;
// impl<const I: usize> EntityRenderCommand for SetSpriteTimeBindGroup<I> {
//     type Param = (SRes<ImageBindGroups>, SQuery<Read<SpriteBatch>>);

//     fn render<'w>(
//         _view: Entity,
//         item: Entity,
//         (material_meta, query_batch): SystemParamItem<'w, '_, Self::Param>,
//         pass: &mut TrackedRenderPass<'w>,
//     ) -> RenderCommandResult {
//         let sprite_batch = query_batch.get(item).unwrap();
//         let image_bind_groups = material_meta.into_inner();

//         pass.set_bind_group(
//             I,
//             image_bind_groups
//                 .values
//                 .get(&Handle::weak(sprite_batch.image_handle_id))
//                 .unwrap(),
//             &[],
//         );
//         RenderCommandResult::Success
//     }
// }

pub struct DrawSpriteBatch;
impl<P: BatchedPhaseItem> RenderCommand<P> for DrawSpriteBatch {
    type Param = (SRes<SpriteMeta>, SQuery<Read<SpriteBatch>>);

    fn render<'w>(
        _view: Entity,
        item: &P,
        (sprite_meta, _query_batch): SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        // let sprite_batch = query_batch.get(item.entity()).unwrap();
        let sprite_meta = sprite_meta.into_inner();
        pass.set_vertex_buffer(0, sprite_meta.vertices.buffer().unwrap().slice(..));
        pass.draw(item.batch_range().as_ref().unwrap().clone(), 0..1);
        RenderCommandResult::Success
    }
}
