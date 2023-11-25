use bevy::prelude::*;

use crate::command_system::events::CommandDispatchEvent;

use super::chunk::{Chunk, ChunkRadius, ChunkRegistry};

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
            if let Ok(parsed_input) = parts[2].parse::<isize>() {
                chunk_radius.radius = parsed_input;
            }
        }
    }
}
