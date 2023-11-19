use bevy::{prelude::*, render::render_resource::PrimitiveTopology};

#[derive(Clone, Copy)]
pub enum BlockFace {
    North,
    South,
    East,
    West,
    Top,
    Bottom,
}

pub fn create_quad(side: BlockFace, offset: Vec3) -> Mesh {
    let result = match side {
        BlockFace::North => create_north_quad(offset),
        BlockFace::South => create_south_quad(offset),
        BlockFace::West => create_west_quad(offset),
        BlockFace::East => create_east_quad(offset),
        BlockFace::Top => create_top_quad(offset),
        BlockFace::Bottom => create_bottom_quad(offset),
    };

    Mesh::new(PrimitiveTopology::TriangleList)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, result.0)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, result.1)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, result.2)
}

pub fn create_north_quad(offset: Vec3) -> (Vec<Vec3>, Vec<Vec2>, Vec<Vec3>) {
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

pub fn create_south_quad(offset: Vec3) -> (Vec<Vec3>, Vec<Vec2>, Vec<Vec3>) {
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

pub fn create_west_quad(offset: Vec3) -> (Vec<Vec3>, Vec<Vec2>, Vec<Vec3>) {
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

pub fn create_east_quad(offset: Vec3) -> (Vec<Vec3>, Vec<Vec2>, Vec<Vec3>) {
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
