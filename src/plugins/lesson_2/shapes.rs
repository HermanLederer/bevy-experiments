use std::f32::consts::PI;

use bevy::{
    prelude::{Color, Mesh},
    render::mesh::{Indices, PrimitiveTopology},
};

pub fn create_circle(num_points: usize, color: Color) -> Mesh {
    let mut star = Mesh::new(PrimitiveTopology::TriangleList);

    // Positions
    let mut v_pos = vec![[0.0, 0.0, 0.0]; num_points + 1];
    for i in 1..=num_points {
        let t = i as f32 / num_points as f32 * PI * 2.0;
        let x = t.sin() * 0.5;
        let y = t.cos() * 0.5;
        v_pos[i] = [x, y, 0.0];
    }
    star.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);

    // Colors
    let v_color: Vec<u32> = vec![color.as_linear_rgba_u32(); num_points + 1];
    star.insert_attribute(Mesh::ATTRIBUTE_COLOR, v_color);

    // Indices
    let mut indices = vec![0, 1, num_points as u32];
    for i in 2..=num_points {
        let iu32 = i as u32;
        indices.extend_from_slice(&[0, iu32, iu32 - 1]);
    }
    star.set_indices(Some(Indices::U32(indices)));

    star
}

pub fn create_star(center_r: f32, spike_delata: f32) -> Mesh {
    // Let's define the mesh for the object we want to draw: a nice star.
    // We will specify here what kind of topology is used to define the mesh,
    // that is, how triangles are built from the vertices. We will use a
    // triangle list, meaning that each vertex of the triangle has to be
    // specified.
    let mut star = Mesh::new(PrimitiveTopology::TriangleList);

    // Vertices need to have a position attribute. We will use the following
    // vertices (I hope you can spot the star in the schema).
    //
    //        1
    //
    //     10   2
    // 9      0      3
    //     8     4
    //        6
    //   7        5
    //
    // These vertices are specificed in 3D space.
    let mut v_pos = vec![[0.0, 0.0, 0.0]];
    for i in 0..10 {
        // Angle of each vertex is 1/10 of TAU, plus PI/2 for positioning vertex 0
        let a = std::f32::consts::FRAC_PI_2 - i as f32 * std::f32::consts::TAU / 10.0;
        // Radius of internal vertices (2, 4, 6, 8, 10) is 100, it's 200 for external
        let r = (1 - i % 2) as f32 * spike_delata + center_r;
        // Add the vertex coordinates
        v_pos.push([r * a.cos(), r * a.sin(), 0.0]);
    }
    // Set the position attribute
    star.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);
    // And a RGB color attribute as well
    let mut v_color: Vec<u32> = vec![Color::BLACK.as_linear_rgba_u32()];
    v_color.extend_from_slice(&[Color::YELLOW.as_linear_rgba_u32(); 10]);
    star.insert_attribute(Mesh::ATTRIBUTE_COLOR, v_color);

    // Now, we specify the indices of the vertex that are going to compose the
    // triangles in our star. Vertices in triangles have to be specified in CCW
    // winding (that will be the front face, colored). Since we are using
    // triangle list, we will specify each triangle as 3 vertices
    //   First triangle: 0, 2, 1
    //   Second triangle: 0, 3, 2
    //   Third triangle: 0, 4, 3
    //   etc
    //   Last triangle: 0, 1, 10
    let mut indices = vec![0, 1, 10];
    for i in 2..=10 {
        indices.extend_from_slice(&[0, i, i - 1]);
    }
    star.set_indices(Some(Indices::U32(indices)));

    star
}
