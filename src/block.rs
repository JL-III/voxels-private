use std::ops::Index;

use bevy::{prelude::*, render::render_resource::PrimitiveTopology};

use crate::element::*;

#[derive(Clone, Copy)]
pub enum BlockFace {
    North,
    South,
    East,
    West,
    Top,
    Bottom,
}

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Block {
    pub uv_mapping: UVMapping,
    pub element: Element,
}

impl Default for Block {
    fn default() -> Self {
        Self {
            uv_mapping: UVMapping([0.0, 0.0]),
            element: Element::Air,
        }
    }
}

impl Block {
    pub fn new(element: Element) -> Self {
        let uv_mapping = match element {
            Element::Air => UVMapping([3.0, 6.0]),
            Element::Stone => UVMapping([0.0, 1.0]),
            Element::Dirt => UVMapping([0.0, 2.0]),
            Element::Grass => UVMapping([0.0, 3.0]),
        };

        Self {
            uv_mapping,
            element,
        }
    }
}

#[derive(Component, Copy, Debug, PartialEq)]
pub struct UVMapping(pub [f32; 2]);

impl Clone for UVMapping {
    fn clone(&self) -> Self {
        *self
    }
}

impl Index<usize> for UVMapping {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

pub fn create_quad(side: BlockFace, offset: Vec3, uv_mapping: UVMapping) -> Mesh {
    let result = match side {
        BlockFace::North => create_north_quad(offset, uv_mapping),
        BlockFace::South => create_south_quad(offset, uv_mapping),
        BlockFace::West => create_west_quad(offset, uv_mapping),
        BlockFace::East => create_east_quad(offset, uv_mapping),
        BlockFace::Top => create_top_quad(offset, uv_mapping),
        BlockFace::Bottom => create_bottom_quad(offset, uv_mapping),
    };

    Mesh::new(PrimitiveTopology::TriangleList)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, result.0)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, result.1)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, result.2)
}

pub fn create_north_quad(offset: Vec3, uv_mapping: UVMapping) -> (Vec<Vec3>, Vec<Vec3>, Vec<Vec2>) {
    let vertices = vec![
        Vec3::new(-0.5, -0.5, 0.5) + offset,
        Vec3::new(0.5, -0.5, 0.5) + offset,
        Vec3::new(0.5, 0.5, 0.5) + offset,
        Vec3::new(-0.5, 0.5, 0.5) + offset,
    ];
    let normals = vec![
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(0.0, 0.0, 1.0),
    ];
    let uvs: Vec<Vec2> = get_texture(uv_mapping[0], uv_mapping[1]);
    (vertices, normals, uvs)
}

pub fn create_south_quad(offset: Vec3, uv_mapping: UVMapping) -> (Vec<Vec3>, Vec<Vec3>, Vec<Vec2>) {
    let vertices = vec![
        Vec3::new(0.5, -0.5, -0.5) + offset,
        Vec3::new(-0.5, -0.5, -0.5) + offset,
        Vec3::new(-0.5, 0.5, -0.5) + offset,
        Vec3::new(0.5, 0.5, -0.5) + offset,
    ];
    let normals = vec![
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 0.0, -1.0),
    ];
    let uvs: Vec<Vec2> = get_texture(uv_mapping[0], uv_mapping[1]);
    (vertices, normals, uvs)
}

pub fn create_west_quad(offset: Vec3, uv_mapping: UVMapping) -> (Vec<Vec3>, Vec<Vec3>, Vec<Vec2>) {
    let vertices = vec![
        Vec3::new(-0.5, -0.5, -0.5) + offset,
        Vec3::new(-0.5, -0.5, 0.5) + offset,
        Vec3::new(-0.5, 0.5, 0.5) + offset,
        Vec3::new(-0.5, 0.5, -0.5) + offset,
    ];
    let normals = vec![
        Vec3::new(-1.0, 0.0, 0.0),
        Vec3::new(-1.0, 0.0, 0.0),
        Vec3::new(-1.0, 0.0, 0.0),
        Vec3::new(-1.0, 0.0, 0.0),
    ];
    let uvs: Vec<Vec2> = get_texture(uv_mapping[0], uv_mapping[1]);
    (vertices, normals, uvs)
}

pub fn create_east_quad(offset: Vec3, uv_mapping: UVMapping) -> (Vec<Vec3>, Vec<Vec3>, Vec<Vec2>) {
    let vertices = vec![
        Vec3::new(0.5, -0.5, 0.5) + offset,
        Vec3::new(0.5, -0.5, -0.5) + offset,
        Vec3::new(0.5, 0.5, -0.5) + offset,
        Vec3::new(0.5, 0.5, 0.5) + offset,
    ];
    let normals = vec![
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
    ];
    let uvs: Vec<Vec2> = get_texture(uv_mapping[0], uv_mapping[1]);
    (vertices, normals, uvs)
}

pub fn create_top_quad(offset: Vec3, uv_mapping: UVMapping) -> (Vec<Vec3>, Vec<Vec3>, Vec<Vec2>) {
    let vertices = vec![
        Vec3::new(-0.5, 0.5, 0.5) + offset,
        Vec3::new(0.5, 0.5, 0.5) + offset,
        Vec3::new(0.5, 0.5, -0.5) + offset,
        Vec3::new(-0.5, 0.5, -0.5) + offset,
    ];
    let normals = vec![
        Vec3::new(0.0, -1.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
    ];
    let uvs: Vec<Vec2> = get_texture(uv_mapping[0], uv_mapping[1]);
    (vertices, normals, uvs)
}

pub fn create_bottom_quad(
    offset: Vec3,
    uv_mapping: UVMapping,
) -> (Vec<Vec3>, Vec<Vec3>, Vec<Vec2>) {
    let vertices = vec![
        Vec3::new(-0.5, -0.5, -0.5) + offset,
        Vec3::new(0.5, -0.5, -0.5) + offset,
        Vec3::new(0.5, -0.5, 0.5) + offset,
        Vec3::new(-0.5, -0.5, 0.5) + offset,
    ];
    let normals = vec![
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    ];

    let uvs: Vec<Vec2> = get_texture(uv_mapping[0], uv_mapping[1]);
    (vertices, normals, uvs)
}

// this function takes the row and column index to identify its texture location in the block atlas
pub fn get_texture(row: f32, column: f32) -> Vec<Vec2> {
    let grid_size = 16.0;
    let mut uvs: Vec<Vec2> = Vec::new();

    let left = column / grid_size;
    let right = (column + 1.0) / grid_size;
    let bottom = row / grid_size;
    let top = (row + 1.0) / grid_size;
    uvs.push(Vec2::new(left, bottom));
    uvs.push(Vec2::new(right, bottom));
    uvs.push(Vec2::new(right, top));
    uvs.push(Vec2::new(left, top));

    uvs
}
