struct View {
    view_proj: mat4x4<f32>,
    world_position: vec3<f32>,
};
@group(0) @binding(0)
var<uniform> view: View;

struct VertexOutput {
    @location(0) uv: vec2<f32>,
    @location(1) t: f32,
    @builtin(position) position: vec4<f32>,
};

@vertex
fn vertex(
    @location(0) vertex_position: vec3<f32>,
    @location(1) vertex_uv: vec2<f32>,
    @location(2) t: f32,
) -> VertexOutput {
    var out: VertexOutput;
    out.uv = vertex_uv;
    out.position = view.view_proj * vec4<f32>(vertex_position, 1.0);
    out.t = t;

    return out;
}

@group(1) @binding(0)
var sprite_texture: texture_2d<f32>;
@group(1) @binding(1)
var sprite_sampler: sampler;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var PI: f32 = 3.14159265358979323846264338327950288;

    var output_color = vec4<f32>(0.0, 0.0, 0.0, 0.0);
    // var t = 0.0;
    var t = in.t;

    output_color[0] = pow(abs(sin(t)), 2.2);
    output_color[1] = pow(abs(sin(t + PI * 0.33333)), 2.2);
    output_color[2] = pow(abs(sin(t + PI * 0.666666)), 2.2);

    var d = 1.0 - distance(vec2<f32>(0.5, 0.5), in.uv);
    d = round(d);
    output_color[3] = d;

    return output_color;
}
