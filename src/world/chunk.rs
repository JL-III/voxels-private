use crate::{
    command_system::events::CommandDispatchEvent,
    player::events::{PlayerMoveEvent, PlayerSpawnEvent},
    world::block::{create_quad, Block, BlockFace},
};
use bevy::{
    asset::{AssetServer, Assets, Handle},
    ecs::{
        component::Component,
        entity::Entity,
        event::{EventReader, EventWriter},
        query::With,
        system::{Commands, Query, Res, ResMut, Resource},
    },
    math::Vec3,
    pbr::{PbrBundle, StandardMaterial},
    prelude::default,
    render::{mesh::Mesh, texture::Image},
    transform::{components::Transform, TransformBundle},
};

use super::{
    block::VertexScale,
    element::Element,
    events::{ChunkCreatedEvent, ChunkEnterEvent},
    mesh_utils::merge_meshes,
};

const CHUNK_WIDTH: usize = 16;
const CHUNK_HEIGHT: usize = 1;
const CHUNK_DEPTH: usize = 16;

#[derive(Component, Clone, Copy, PartialEq)]
pub struct Chunk {
    chunk_x: isize,
    chunk_z: isize,
    blocks: [[[Block; CHUNK_WIDTH]; CHUNK_HEIGHT]; CHUNK_DEPTH],
}

pub fn setup_initial_chunks(
    mut chunk_registry: ResMut<ChunkRegistry>,
    mut commands: Commands,
    mut player_spawned_event: EventReader<PlayerSpawnEvent>,
    mut chunk_create_event_write: EventWriter<ChunkCreatedEvent>,
) {
    for event in player_spawned_event.read() {
        let chunks = get_surrounding_chunks(convert_to_chunk_location(event.position.x), convert_to_chunk_location(event.position.z), 3);
        for chunk in chunks.iter() {
            if !chunk_registry.chunks.contains(chunk) {
                chunk_registry.chunks.push(*chunk);
                let chunk_transform = commands.spawn((
                    *chunk,
                    TransformBundle {
                        local: {
                            Transform::from_xyz(chunk.chunk_x as f32, 0.0, chunk.chunk_z as f32)
                        },
                        ..Default::default()
                    },
                ));
                chunk_create_event_write.send(ChunkCreatedEvent {
                    chunk: *chunk,
                    chunk_id: chunk_transform.id(),
                });
            }
        }
    }
}

pub fn chunk_enter_listener(
    mut chunk_registry: ResMut<ChunkRegistry>,
    mut commands: Commands,
    mut chunk_enter_event_reader: EventReader<ChunkEnterEvent>,
    mut chunk_create_event_write: EventWriter<ChunkCreatedEvent>,
) {
    for event in chunk_enter_event_reader.read() {
        let chunks = get_surrounding_chunks(event.chunk_x, event.chunk_z, 3);
        for chunk in chunks.iter() {
            if !chunk_registry.chunks.contains(chunk) {
                chunk_registry.chunks.push(*chunk);
                let chunk_transform = commands.spawn((
                    *chunk,
                    TransformBundle {
                        local: {
                            Transform::from_xyz(chunk.chunk_x as f32, 0.0, chunk.chunk_z as f32)
                        },
                        ..Default::default()
                    },
                ));
                chunk_create_event_write.send(ChunkCreatedEvent {
                    chunk: *chunk,
                    chunk_id: chunk_transform.id(),
                });
            }
        }
    }
}

// Takes in the chunk_x and chunk_z values to find the chunks
pub fn get_surrounding_chunks(x: isize, z: isize, radius: isize) -> Vec<Chunk> {
    let mut chunks = Vec::new();
    let radius_squared = radius.pow(2);

    for dx in -radius..=radius {
        for dz in -radius..=radius {
            if dx.pow(2) + dz.pow(2) <= radius_squared {
                let chunk_x = x + dx;
                let chunk_z = z + dz;
                chunks.push(generate_chunk(chunk_x, chunk_z));
            }
        }
    }

    chunks
}

pub fn generate_chunk(x: isize, z: isize) -> Chunk {
    let mut chunk = Chunk {
        chunk_x: x,
        chunk_z: z,
        blocks: [[[Block::default(); CHUNK_WIDTH]; CHUNK_HEIGHT]; CHUNK_DEPTH],
    };
    for x in 0..CHUNK_WIDTH {
        for y in 0..CHUNK_HEIGHT {
            for z in 0..CHUNK_DEPTH {
                chunk.blocks[x][y][z] = Block::new(get_random_element(y));
            }
        }
    }
    chunk
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
                chunk_event.chunk.chunk_x as f32,
                0.0,
                chunk_event.chunk.chunk_z as f32,
            ),
            ..default()
        });
    }
}

pub fn player_move_event_listener(
    mut player_move_event_reader: EventReader<PlayerMoveEvent>,
    mut enter_chunk_event_writer: EventWriter<ChunkEnterEvent>,
) {
    for event in player_move_event_reader.read() {
        let starting_chunk_x = (event.starting_position.x / 16.0).floor() as isize;
        let starting_chunk_z = (event.starting_position.z / 16.0).floor() as isize;

        let final_chunk_x = (event.final_position.x / 16.0).floor() as isize;
        let final_chunk_z = (event.final_position.z / 16.0).floor() as isize;

        if starting_chunk_x != final_chunk_x || starting_chunk_z != final_chunk_z {
            enter_chunk_event_writer.send(ChunkEnterEvent {
                chunk_x: final_chunk_x,
                chunk_z: final_chunk_z,
            });
        }
    }
}

fn gen_meshes(scale: f32, chunk_event: &ChunkCreatedEvent) -> Vec<Mesh> {
    let mut gen_meshes: Vec<Mesh> = Vec::new();

    for x in 0..CHUNK_WIDTH {
        for y in 0..CHUNK_HEIGHT {
            for z in 0..CHUNK_DEPTH {
                let block = chunk_event.chunk.blocks[x][y][z];
                let mesh_location = Vec3::new(
                    chunk_event.chunk.chunk_x as f32 * 16.0 + x as f32,
                    y as f32,
                    chunk_event.chunk.chunk_z as f32 * 16.0 + z as f32,
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

fn get_random_element(y: usize) -> Element {
    match y {
        _ if y == 10 => Element::Grass,
        _ if y < 10 && y > 5 => Element::Dirt,
        _ if y <= 5 => Element::Stone,
        _ => Element::Air,
    }
}

fn convert_to_chunk_location(location: f32) -> isize {
    (location / 16.0).floor() as isize
}

#[derive(Resource)]
pub struct ChunkRegistry {
    pub chunks: Vec<Chunk>,
}

pub fn despawn_chunks_command(
    mut commands: Commands,
    mut chunk_registry: ResMut<ChunkRegistry>,
    chunk_query: Query<Entity, With<Chunk>>,
    mut command_dispatch_event_reader: EventReader<CommandDispatchEvent>,
) {
    for event in command_dispatch_event_reader.read() {
        let parts: Vec<&str> = event.command.split_whitespace().collect();
        if parts.len() == 2 && parts[0] == "/chunk" && parts[1] == "despawn" {
            for entity in chunk_query.iter() {
                commands.entity(entity).despawn();
                chunk_registry.chunks.clear();
            }
        }
    }
}
