use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

pub const POINT_0: Vec3 = Vec3::new(-0.5, -0.5, 0.5);
pub const POINT_1: Vec3 = Vec3::new(0.5, -0.5, 0.5);
pub const POINT_2: Vec3 = Vec3::new(0.5, -0.5, -0.5);
pub const POINT_3: Vec3 = Vec3::new(-0.5, -0.5, -0.5);
pub const POINT_4: Vec3 = Vec3::new(-0.5, 0.5, 0.5);
pub const POINT_5: Vec3 = Vec3::new(0.5, 0.5, 0.5);
pub const POINT_6: Vec3 = Vec3::new(-0.5, -0.5, 0.5);
pub const POINT_7: Vec3 = Vec3::new(-0.5, 0.5, -0.5);


pub fn create_simple_quad() -> Mesh {
    // Create a new mesh using a triangle list topology, where each set of 3 vertices composes a triangle.
    Mesh::new(PrimitiveTopology::TriangleList)
        // Add 4 vertices, each with its own position attribute (coordinate in
        // 3D space), for each of the corners of the parallelogram.
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![
                POINT_4,
                POINT_5,
                POINT_1,
                POINT_0
            ],
        )
        // Assign a UV coordinate to each vertex.
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_UV_0,
            vec![[1.0, 1.0], [0.0, 1.0], [0.0, 0.0], [1.0, 0.0]],
        )
        // Assign normals (everything points outwards)
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            vec![
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
            ],
        )
        // After defining all the vertices and their attributes, build each triangle using the
        // indices of the vertices that make it up in a counter-clockwise order.
        .with_indices(Some(Indices::U32(vec![
            3, 1, 0, // First triangle
            3, 2, 1, // Second triangle
        ])))
}
