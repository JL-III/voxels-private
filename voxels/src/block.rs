use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};

#[derive(Clone, Copy)]
pub enum BlockFace {
    Front,
    Back,
    Left,
    Right,
    Top,
    Bottom,
}

pub fn create_quad(side: BlockFace, offset: Vec3) -> Mesh {
    
    let result = match side {
        BlockFace::Front => create_front_quad(offset),
        BlockFace::Back => create_back_quad(offset),
        BlockFace::Left => create_left_quad(offset),
        BlockFace::Right => create_right_quad(offset),
        BlockFace::Top => create_top_quad(offset),
        BlockFace::Bottom => create_bottom_quad(offset),
    };

    Mesh::new(PrimitiveTopology::TriangleList)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, result.0)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, result.1)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, result.2)
        .with_indices(Some(Indices::U32(result.3)))
}

pub fn create_front_quad(offset: Vec3) -> (Vec<Vec3>, Vec<Vec2>, Vec<Vec3>, Vec<u32>) {
    let vertices = vec![
        Vec3::new(-0.5, -0.5, 0.5) + offset,
        Vec3::new(0.5, -0.5, 0.5) + offset,
        Vec3::new(0.5, 0.5, 0.5) + offset,
        Vec3::new(-0.5, 0.5, 0.5) + offset,
    ];
    let uvs = vec![
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(1.0, 1.0),
        Vec2::new(0.0, 1.0),
    ];
    let normals = vec![
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(0.0, 0.0, 1.0),
    ];
    let indices = vec![
        0, 1, 2, 2, 3, 0, // Front face
    ];
    (vertices, uvs, normals, indices)
}

pub fn create_back_quad(offset: Vec3) -> (Vec<Vec3>, Vec<Vec2>, Vec<Vec3>, Vec<u32>) {
    let vertices = vec![
        Vec3::new(0.5, -0.5, -0.5) + offset,
        Vec3::new(-0.5, -0.5, -0.5) + offset,
        Vec3::new(-0.5, 0.5, -0.5) + offset,
        Vec3::new(0.5, 0.5, -0.5) + offset,
    ];
    let uvs = vec![
        Vec2::new(1.0, 0.0),
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, 1.0),
        Vec2::new(1.0, 1.0),
    ];
    let normals = vec![
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 0.0, -1.0),
    ];
    let indices = vec![
        4, 5, 6, 6, 7, 4, // Back face
    ];
    (vertices, uvs, normals, indices)
}

pub fn create_left_quad(offset: Vec3) -> (Vec<Vec3>, Vec<Vec2>, Vec<Vec3>, Vec<u32>) {
    let vertices = vec![
        Vec3::new(-0.5, -0.5, -0.5) + offset,
        Vec3::new(-0.5, -0.5, 0.5) + offset,
        Vec3::new(-0.5, 0.5, 0.5) + offset,
        Vec3::new(-0.5, 0.5, -0.5) + offset,
    ];
    let uvs = vec![
        Vec2::new(1.0, 0.0),
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, 1.0),
        Vec2::new(1.0, 1.0),
    ];
    let normals = vec![
        Vec3::new(-1.0, 0.0, 0.0),
        Vec3::new(-1.0, 0.0, 0.0),
        Vec3::new(-1.0, 0.0, 0.0),
        Vec3::new(-1.0, 0.0, 0.0),
    ];
    let indices = vec![
        8, 9, 10, 10, 11, 8, // Left face
    ];
    (vertices, uvs, normals, indices)
}

pub fn create_right_quad(offset: Vec3) -> (Vec<Vec3>, Vec<Vec2>, Vec<Vec3>, Vec<u32>) {
    let vertices = vec![
        Vec3::new(0.5, -0.5, 0.5) + offset,
        Vec3::new(0.5, -0.5, -0.5) + offset,
        Vec3::new(0.5, 0.5, -0.5) + offset,
        Vec3::new(0.5, 0.5, 0.5) + offset,
    ];
    let uvs = vec![
        Vec2::new(1.0, 0.0),
        Vec2::new(0.0, 0.0),
        Vec2::new(0.0, 1.0),
        Vec2::new(1.0, 1.0),
    ];
    let normals = vec![
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
    ];
    let indices = vec![
        12, 13, 14, 14, 15, 12, // Right face
    ];
    (vertices, uvs, normals, indices)
}

pub fn create_top_quad(offset: Vec3) -> (Vec<Vec3>, Vec<Vec2>, Vec<Vec3>, Vec<u32>) {
    let vertices = vec![
        Vec3::new(-0.5, 0.5, 0.5) + offset,
        Vec3::new(0.5, 0.5, 0.5) + offset,
        Vec3::new(0.5, 0.5, -0.5) + offset,
        Vec3::new(-0.5, 0.5, -0.5) + offset,
    ];
    let uvs = vec![
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        Vec2::new(1.0, 1.0),
        Vec2::new(0.0, 1.0),
    ];
    let normals = vec![
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    ];
    let indices = vec![
        16, 17, 18, 18, 19, 16, // Top face
    ];
    (vertices, uvs, normals, indices)
}

pub fn create_bottom_quad(offset: Vec3) -> (Vec<Vec3>, Vec<Vec2>, Vec<Vec3>, Vec<u32>) {
    let vertices = vec![
        Vec3::new(-0.5, -0.5, -0.5) + offset,
        Vec3::new(0.5, -0.5, -0.5) + offset,
        Vec3::new(0.5, -0.5, 0.5) + offset,
        Vec3::new(-0.5, -0.5, 0.5) + offset,
    ];
    let uvs = vec![
        Vec2::new(1.0, 1.0),
        Vec2::new(0.0, 1.0),
        Vec2::new(0.0, 0.0),
        Vec2::new(1.0, 0.0),
        ];
    let normals = vec![
        Vec3::new(0.0, -1.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
    ];

    let indices = vec![
        20, 21, 22, 22, 23, 20, // Bottom face
    ];
    (vertices, uvs, normals, indices)
}
