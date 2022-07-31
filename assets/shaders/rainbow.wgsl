// #import bevy_sprite::mesh2d_types
// #import bevy_sprite::mesh2d_view_bindings

@group(1) @binding(0)
var<uniform> t: f32;

// @group(2) @binding(0)
// var<uniform> mesh: Mesh2d;

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    #import bevy_sprite::mesh2d_vertex_output
};

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    var PI: f32 = 3.14159265358979323846264338327950288;

    var output_color = vec4<f32>(0.0, 0.0, 0.0, 0.0);

    output_color[0] = pow(abs(sin(t)), 2.2);
    output_color[1] = pow(abs(sin(t + PI * 0.33333)), 2.2);
    output_color[2] = pow(abs(sin(t + PI * 0.666666)), 2.2);

    var d = 1.0 - distance(vec2<f32>(0.5, 0.5), in.uv);
    d = round(d);
    output_color[3] = d;

    return output_color;
}
