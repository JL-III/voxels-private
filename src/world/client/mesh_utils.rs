use bevy::asset::{AssetServer, Assets, Handle};
use bevy::ecs::event::EventReader;
use bevy::ecs::system::{Commands, Res, ResMut};
use bevy::math::{Vec2, Vec3};
use bevy::pbr::{PbrBundle, StandardMaterial};
use bevy::prelude::default;
use bevy::render::mesh::Indices;
use bevy::render::mesh::{Mesh, VertexAttributeValues};
use bevy::render::render_resource::PrimitiveTopology;
use bevy::render::texture::Image;
use bevy::transform::components::Transform;
use std::hash::Hash;
use std::hash::Hasher;

use crate::world::block::{create_quad, BlockFace, VertexScale};
use crate::world::chunk::{CHUNK_DEPTH, CHUNK_HEIGHT, CHUNK_WIDTH};
use crate::world::element::Element;
use crate::world::events::ChunkCreatedEvent;

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

pub fn render(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut chunk_create_event_reader: EventReader<ChunkCreatedEvent>,
    vertex_scale: Res<VertexScale>,
    asset_server: Res<AssetServer>,
) {
    for chunk_event in chunk_create_event_reader.read() {
        let block_atlas: Handle<Image> = asset_server.load("sprites/blockatlas.png");
        let combined_mesh = merge_meshes(gen_meshes(vertex_scale.scale, chunk_event));
        commands.entity(chunk_event.chunk_id).insert(PbrBundle {
            mesh: meshes.add(combined_mesh.clone()),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(block_atlas.clone()),
                unlit: false,
                ..default()
            }),
            transform: Transform::from_xyz(
                chunk_event.chunk.chunk_x,
                chunk_event.chunk.chunk_y,
                chunk_event.chunk.chunk_z,
            ),
            ..default()
        });
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
                    chunk_event.chunk.chunk_x * 15.0 + x as f32,
                    chunk_event.chunk.chunk_y * 15.0 + y as f32,
                    chunk_event.chunk.chunk_z * 15.0 + z as f32,
                );
                // exempt air from needing a mesh
                if block.element == Element::Air {
                    continue;
                };
                if x == CHUNK_WIDTH - 1
                    || chunk_event.chunk.blocks[x + 1][y][z].element == Element::Air
                {
                    gen_meshes.push(create_quad(
                        scale,
                        BlockFace::East,
                        mesh_location,
                        block.uv_mapping,
                    ));
                }
                if z == CHUNK_DEPTH - 1
                    || chunk_event.chunk.blocks[x][y][z + 1].element == Element::Air
                {
                    gen_meshes.push(create_quad(
                        scale,
                        BlockFace::North,
                        mesh_location,
                        block.uv_mapping,
                    ));
                }
                if y == CHUNK_HEIGHT - 1
                    || chunk_event.chunk.blocks[x][y + 1][z].element == Element::Air
                {
                    gen_meshes.push(create_quad(
                        scale,
                        BlockFace::Top,
                        mesh_location,
                        block.uv_mapping,
                    ));
                }
                if y == 0 || chunk_event.chunk.blocks[x][y - 1][z].element == Element::Air {
                    gen_meshes.push(create_quad(
                        scale,
                        BlockFace::Bottom,
                        mesh_location,
                        block.uv_mapping,
                    ));
                }
                if z == 0 || chunk_event.chunk.blocks[x][y][z - 1].element == Element::Air {
                    gen_meshes.push(create_quad(
                        scale,
                        BlockFace::South,
                        mesh_location,
                        block.uv_mapping,
                    ));
                }
                if x == 0 || chunk_event.chunk.blocks[x - 1][y][z].element == Element::Air {
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
