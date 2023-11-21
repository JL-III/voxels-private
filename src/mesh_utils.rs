use bevy::math::{Vec2, Vec3};
use bevy::render::mesh::Indices;
use bevy::render::mesh::{Mesh, VertexAttributeValues};
use bevy::render::render_resource::PrimitiveTopology;
use std::hash::Hash;
use std::hash::Hasher;

use crate::block::Block;

#[derive(Clone)]
struct VertexData {
    position: Vec3,
    normal: Vec3,
    uv: Vec2,
}

impl PartialEq for VertexData {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position && self.normal == other.normal && self.uv == other.uv
    }
}

impl Eq for VertexData {}

impl Hash for VertexData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        hash_float_array(&self.position.to_array(), state);
        hash_float_array(&self.normal.to_array(), state);
        hash_float_array(&self.uv.to_array(), state);
    }
}

fn hash_float_array<H: Hasher>(arr: &[f32], state: &mut H) {
    for &num in arr {
        num.to_bits().hash(state);
    }
}

pub fn merge_meshes(meshes: Vec<Mesh>, block: &Block) -> Mesh {
    let mut combined_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    let mut offset = 0;
    for mesh in meshes {
        // Gather the vertex attributes for positions, normals, and uvs
        if let Some(VertexAttributeValues::Float32x3(positions)) =
            mesh.attribute(Mesh::ATTRIBUTE_POSITION)
        {
            vertices.extend_from_slice(positions);
        }
        if let Some(VertexAttributeValues::Float32x3(norms)) =
            mesh.attribute(Mesh::ATTRIBUTE_NORMAL)
        {
            normals.extend_from_slice(norms);
        }

        uvs.extend_from_slice(&get_texture(block.uv_mapping[0], block.uv_mapping[1]));

        indices.push(offset);
        indices.push(1 + offset);
        indices.push(2 + offset);
        indices.push(2 + offset);
        indices.push(3 + offset);
        indices.push(offset);

        // Increment the offset by the number of vertices in the current quad (which should be 4 for a quad)
        offset += 4;
    }

    // Now you can set the vertices, normals, uvs, and indices for the combined mesh
    combined_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    combined_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    combined_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    combined_mesh.set_indices(Some(Indices::U32(indices)));

    combined_mesh
}

fn get_texture(row: f32, column: f32) -> Vec<[f32; 2]> {
    let grid_size = 16.0;
    let mut uvs = Vec::new();

    let left = column / grid_size;
    let right = (column + 1.0) / grid_size;
    let bottom = row / grid_size;
    let top = (row + 1.0) / grid_size;
    uvs.push([left, bottom]);
    uvs.push([right, bottom]);
    uvs.push([right, top]);
    uvs.push([left, top]);

    uvs
}
