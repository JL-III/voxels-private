use bevy::{
    prelude::*,
    render::render_resource::PrimitiveTopology,
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
}

pub fn create_front_quad(offset: Vec3) -> (Vec<Vec3>, Vec<Vec2>, Vec<Vec3>) {
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
    (vertices, uvs, normals)
}

pub fn create_back_quad(offset: Vec3) -> (Vec<Vec3>, Vec<Vec2>, Vec<Vec3>) {
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
    (vertices, uvs, normals)
}

pub fn create_left_quad(offset: Vec3) -> (Vec<Vec3>, Vec<Vec2>, Vec<Vec3>) {
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
    (vertices, uvs, normals)
}

pub fn create_right_quad(offset: Vec3) -> (Vec<Vec3>, Vec<Vec2>, Vec<Vec3>) {
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
    (vertices, uvs, normals)
}

pub fn create_top_quad(offset: Vec3) -> (Vec<Vec3>, Vec<Vec2>, Vec<Vec3>) {
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
    (vertices, uvs, normals)
}

pub fn create_bottom_quad(offset: Vec3) -> (Vec<Vec3>, Vec<Vec2>, Vec<Vec3>) {
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

    (vertices, uvs, normals)
}
