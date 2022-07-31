// #import bevy_sprite::mesh2d_types
// #import bevy_sprite::mesh2d_view_bindings

@group(1) @binding(0)
var<uniform> color: vec4<f32>;

// @group(2) @binding(0)
// var<uniform> mesh: Mesh2d;

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    #import bevy_sprite::mesh2d_vertex_output
};

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    var output_color = color;

    var d = 1.0 - distance(vec2<f32>(0.5, 0.5), in.uv);
    d = round(d);
    output_color[3] = d;

    return output_color;
}
