use bevy::prelude::*;

use crate::{
    command_system::events::CommandDispatchEvent,
    player::controls::Player,
    world::{
        block::{create_quad, Block},
        mesh_utils::merge_meshes,
    },
};

use super::{
    block::{BlockFace, VertexScale},
    chunk::{Chunk, ChunkRadius, ChunkRegistry},
    element::Element,
};

pub fn chunk_despawn_command(
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

pub fn chunk_radius_command(
    mut chunk_radius: ResMut<ChunkRadius>,
    mut command_dispatch_event_reader: EventReader<CommandDispatchEvent>,
) {
    for event in command_dispatch_event_reader.read() {
        let parts: Vec<&str> = event.command.split_whitespace().collect();
        if parts.len() == 3 && parts[0] == "/chunk" && parts[1] == "radius" {
            if let Ok(parsed_input) = parts[2].parse::<i32>() {
                chunk_radius.radius = parsed_input;
            }
        }
    }
}

pub fn spawn_cube_command(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    vertex_scale: Res<VertexScale>,
    asset_server: Res<AssetServer>,
    mut command_dispatch_event_reader: EventReader<CommandDispatchEvent>,
    transform_query: Query<&Transform, With<Player>>,
) {
    for event in command_dispatch_event_reader.read() {
        let block_atlas: Handle<Image> = asset_server.load("sprites/blockatlas.png");
        let sides = vec![
            BlockFace::Top,
            BlockFace::Bottom,
            BlockFace::East,
            BlockFace::West,
            BlockFace::North,
            BlockFace::South,
        ];
        let parts: Vec<&str> = event.command.split_whitespace().collect();
        if parts.len() == 2 && parts[0] == "/block" {
            let mut element = Element::Air;
            match Element::from_str(parts[1]) {
                Some(Element::Air) => {}
                Some(Element::Dirt) => element = Element::Dirt,
                Some(Element::Grass) => element = Element::Grass,
                Some(Element::Stone) => element = Element::Stone,
                _ => {
                    println!("'{}' is not a valid element.", parts[1]);
                    return;
                }
            }
            if let Ok(transform) = transform_query.get_single() {
                println!("inside transform");
                let mut combined_mesh: Vec<Mesh> = Vec::new();
                let block = Block::new(element);
                for side in sides {
                    combined_mesh.push(create_quad(
                        vertex_scale.scale,
                        side,
                        Vec3 {
                            x: 0.0,
                            y: 0.0,
                            z: 1.0,
                        },
                        block.uv_mapping,
                    ));
                }
                println!("number of meshes in combined_mesh: {}", combined_mesh.len());
                println!(
                    "transform translation  x: {}, y: {}, z: {}",
                    transform.translation.x, transform.translation.y, transform.translation.z,
                );
                commands.spawn(PbrBundle {
                    mesh: meshes.add(merge_meshes(combined_mesh)),
                    material: materials.add(StandardMaterial {
                        base_color_texture: Some(block_atlas.clone()),
                        unlit: false,
                        ..default()
                    }),
                    transform: Transform::from_xyz(
                        transform.translation.x,
                        transform.translation.y,
                        transform.translation.z + 1.0,
                    ),
                    ..default()
                });
            } else {
                warn!("player transform not found!");
            }
        }
    }
}
