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

#[derive(Component)]
pub struct Block {
    pub uv_mapping: UVMapping,
    pub element: Element,
}

impl Block {
    pub fn new(element: Element) -> Self {
        let uv_mapping = match element {
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

pub struct UVMapping([f32; 2]);

impl Index<usize> for UVMapping {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

// i actually think uv mapping will have to exist here since
// the different sides of a block will be mapped to different things
// depending on the block

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
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, result.1)
}

pub fn create_north_quad(offset: Vec3) -> (Vec<Vec3>, Vec<Vec3>) {
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
    (vertices, normals)
}

pub fn create_south_quad(offset: Vec3) -> (Vec<Vec3>, Vec<Vec3>) {
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
    (vertices, normals)
}

pub fn create_west_quad(offset: Vec3) -> (Vec<Vec3>, Vec<Vec3>) {
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
    (vertices, normals)
}

pub fn create_east_quad(offset: Vec3) -> (Vec<Vec3>, Vec<Vec3>) {
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
    (vertices, normals)
}

pub fn create_top_quad(offset: Vec3) -> (Vec<Vec3>, Vec<Vec3>) {
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
    (vertices, normals)
}

pub fn create_bottom_quad(offset: Vec3) -> (Vec<Vec3>, Vec<Vec3>) {
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

    (vertices, normals)
}
