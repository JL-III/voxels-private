use bevy::math::{Vec2, Vec3};
use bevy::render::mesh::Indices;
use bevy::render::mesh::{Mesh, VertexAttributeValues};
use bevy::render::render_resource::PrimitiveTopology;
use std::hash::Hash;
use std::hash::Hasher;

use super::block::{create_quad, BlockFace};
use super::chunk::{CHUNK_WIDTH, CHUNK_HEIGHT, CHUNK_DEPTH};
use super::events::ChunkCreatedEvent;

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

pub fn merge_meshes(meshes: Vec<Mesh>) -> Mesh {
    let mut combined_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    let mut offset = 0;
    for mesh in meshes {
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
        if let Some(VertexAttributeValues::Float32x2(texture_coords)) =
            mesh.attribute(Mesh::ATTRIBUTE_UV_0)
        {
            uvs.extend_from_slice(texture_coords);
        }

        indices.push(offset);
        indices.push(1 + offset);
        indices.push(2 + offset);
        indices.push(2 + offset);
        indices.push(3 + offset);
        indices.push(offset);

        offset += 4;
    }

    combined_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    combined_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    combined_mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    combined_mesh.set_indices(Some(Indices::U32(indices)));

    combined_mesh
}

pub fn gen_meshes(scale: f32, chunk_event: &ChunkCreatedEvent) -> Vec<Mesh> {
    let mut gen_meshes: Vec<Mesh> = Vec::new();

    for x in 0..CHUNK_WIDTH {
        for y in 0..CHUNK_HEIGHT {
            for z in 0..CHUNK_DEPTH {
                let block = chunk_event.chunk.blocks[x][y][z];
                let mesh_location = Vec3::new(
                    chunk_event.chunk.chunk_x as f32 * 15.0 + x as f32,
                    y as f32,
                    chunk_event.chunk.chunk_z as f32 * 15.0 + z as f32,
                );
                if x == CHUNK_WIDTH - 1 {
                    gen_meshes.push(create_quad(
                        scale,
                        BlockFace::East,
                        mesh_location,
                        block.uv_mapping,
                    ));
                }
                if z == CHUNK_DEPTH - 1 {
                    gen_meshes.push(create_quad(
                        scale,
                        BlockFace::North,
                        mesh_location,
                        block.uv_mapping,
                    ));
                }
                if y == CHUNK_HEIGHT - 1 {
                    gen_meshes.push(create_quad(
                        scale,
                        BlockFace::Top,
                        mesh_location,
                        block.uv_mapping,
                    ));
                }
                if y == 0 {
                    gen_meshes.push(create_quad(
                        scale,
                        BlockFace::Bottom,
                        mesh_location,
                        block.uv_mapping,
                    ));
                }
                if z == 0 {
                    gen_meshes.push(create_quad(
                        scale,
                        BlockFace::South,
                        mesh_location,
                        block.uv_mapping,
                    ));
                }
                if x == 0 {
                    gen_meshes.push(create_quad(
                        scale,
                        BlockFace::West,
                        mesh_location,
                        block.uv_mapping,
                    ));
                }
            }
        }
    }
    gen_meshes
}