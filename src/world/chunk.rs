use crate::{
    player::events::{PlayerMoveEvent, PlayerSpawnEvent},
    world::block::Block,
};
use bevy::{
    asset::{AssetServer, Assets, Handle},
    ecs::{
        component::Component,
        event::{EventReader, EventWriter},
        system::{Commands, Res, ResMut, Resource},
    },
    pbr::{PbrBundle, StandardMaterial},
    prelude::default,
    render::{mesh::Mesh, texture::Image},
    transform::{components::Transform, TransformBundle},
};

use super::{
    block::VertexScale,
    element::Element,
    events::{ChunkCreatedEvent, ChunkEnterEvent},
    mesh_utils::{gen_meshes, merge_meshes},
};

pub const CHUNK_WIDTH: usize = 16;
pub const CHUNK_HEIGHT: usize = 16;
pub const CHUNK_DEPTH: usize = 16;

#[derive(Component, Clone, Copy, PartialEq)]
pub struct Chunk {
    pub chunk_x: isize,
    pub chunk_z: isize,
    pub blocks: [[[Block; CHUNK_WIDTH]; CHUNK_HEIGHT]; CHUNK_DEPTH],
}

pub fn setup_initial_chunks(
    mut chunk_registry: ResMut<ChunkRegistry>,
    chunk_radius: Res<ChunkRadius>,
    mut commands: Commands,
    mut player_spawned_event: EventReader<PlayerSpawnEvent>,
    mut chunk_create_event_write: EventWriter<ChunkCreatedEvent>,
) {
    for event in player_spawned_event.read() {
        let chunks = get_surrounding_chunks(
            convert_to_chunk_location(event.position.x),
            convert_to_chunk_location(event.position.z),
            chunk_radius.radius,
        );
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
                    registry_size: chunk_registry.chunks.len(),
                });
            }
        }
    }
}

pub fn chunk_enter_listener(
    mut chunk_registry: ResMut<ChunkRegistry>,
    chunk_radius: Res<ChunkRadius>,
    mut commands: Commands,
    mut chunk_enter_event_reader: EventReader<ChunkEnterEvent>,
    mut chunk_create_event_write: EventWriter<ChunkCreatedEvent>,
) {
    for event in chunk_enter_event_reader.read() {
        let chunks = get_surrounding_chunks(event.chunk_x, event.chunk_z, chunk_radius.radius);
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
                    registry_size: chunk_registry.chunks.len(),
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
                chunk.blocks[x][y][z] = Block::new(get_random_element(x, y, z));
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

use noise::{NoiseFn, Perlin};

fn get_random_element(x: usize, y: usize, z: usize) -> Element {
    let perlin = Perlin::new(1);
    let noise_value = perlin.get([x as f64 * 0.1, y as f64 * 0.1, z as f64 * 0.1]);

    // Adjust y based on noise (this is just an example, adjust as needed)
    let adjusted_y = y as f64 + noise_value * 10.0;

    match adjusted_y as usize {
        _ if adjusted_y > 10.0 => Element::Air,
        _ if adjusted_y <= 10.0 && adjusted_y > 5.0 => Element::Dirt,
        _ if adjusted_y <= 5.0 => Element::Stone,
        _ => Element::Grass,
    }
}

fn convert_to_chunk_location(location: f32) -> isize {
    (location / 16.0).floor() as isize
}

#[derive(Resource)]
pub struct ChunkRegistry {
    pub chunks: Vec<Chunk>,
}

#[derive(Resource)]
pub struct ChunkRadius {
    pub radius: isize,
}
