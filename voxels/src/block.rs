use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

pub fn create_simple_cube(offset: Vec3) -> Mesh {
    // Define the 8 unique vertices of the cube
    let vertices = vec![
        // Front face
        Vec3::new(-0.5, -0.5, 0.5) + offset,
        Vec3::new(0.5, -0.5, 0.5) + offset,
        Vec3::new(0.5, 0.5, 0.5) + offset,
        Vec3::new(-0.5, 0.5, 0.5) + offset,
        // Back face
        Vec3::new(0.5, -0.5, -0.5) + offset,
        Vec3::new(-0.5, -0.5, -0.5) + offset,
        Vec3::new(-0.5, 0.5, -0.5) + offset,
        Vec3::new(0.5, 0.5, -0.5) + offset,
        // Left face
        Vec3::new(-0.5, -0.5, -0.5) + offset,
        Vec3::new(-0.5, -0.5, 0.5) + offset,
        Vec3::new(-0.5, 0.5, 0.5) + offset,
        Vec3::new(-0.5, 0.5, -0.5) + offset,
        // Right face
        Vec3::new(0.5, -0.5, 0.5) + offset,
        Vec3::new(0.5, -0.5, -0.5) + offset,
        Vec3::new(0.5, 0.5, -0.5) + offset,
        Vec3::new(0.5, 0.5, 0.5) + offset,
        // Top face
        Vec3::new(-0.5, 0.5, 0.5) + offset,
        Vec3::new(0.5, 0.5, 0.5) + offset,
        Vec3::new(0.5, 0.5, -0.5) + offset,
        Vec3::new(-0.5, 0.5, -0.5) + offset,
        // Bottom face
        Vec3::new(-0.5, -0.5, -0.5) + offset,
        Vec3::new(0.5, -0.5, -0.5) + offset,
        Vec3::new(0.5, -0.5, 0.5) + offset,
        Vec3::new(-0.5, -0.5, 0.5) + offset,
    ];

    // Define the indices for the 12 triangles (6 faces * 2 triangles per face)
    let indices = vec![
        // Front face
        0, 1, 2, 2, 3, 0, // Back face
        4, 5, 6, 6, 7, 4, // Left face
        8, 9, 10, 10, 11, 8, // Right face
        12, 13, 14, 14, 15, 12, // Top face
        16, 17, 18, 18, 19, 16, // Bottom face
        20, 21, 22, 22, 23, 20,
    ];

    // Define UV coordinates for each vertex
    let uvs = vec![
        // Front face UVs
        [0.0, 0.0],
        [1.0, 0.0],
        [1.0, 1.0],
        [0.0, 1.0],
        // Back face UVs
        [1.0, 0.0],
        [0.0, 0.0],
        [0.0, 1.0],
        [1.0, 1.0],
        // Left face UVs
        [1.0, 0.0],
        [0.0, 0.0],
        [0.0, 1.0],
        [1.0, 1.0],
        // Right face UVs
        [1.0, 0.0],
        [0.0, 0.0],
        [0.0, 1.0],
        [1.0, 1.0],
        // Top face UVs
        [0.0, 0.0],
        [1.0, 0.0],
        [1.0, 1.0],
        [0.0, 1.0],
        // Bottom face UVs
        [1.0, 1.0],
        [0.0, 1.0],
        [0.0, 0.0],
        [1.0, 0.0],
    ];

    let normals = vec![
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, -1.0],
        [0.0, 0.0, -1.0],
        [0.0, 0.0, -1.0],
        [0.0, 0.0, -1.0],
        [-1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0],
        [-1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, -1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
        [0.0, 1.0, 0.0],
    ];

    // Create and return the mesh
    Mesh::new(PrimitiveTopology::TriangleList)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
        .with_indices(Some(Indices::U32(indices)))
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    // Additional attributes like normals and UVs can be added similarly
}
