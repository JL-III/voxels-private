use bevy::prelude::*;
use bevy_renet::renet::RenetServer;

use crate::{
    world::{chunk::Chunk, events::ChunkCreatedEvent},
    ServerChannel,
};

pub fn send_chunk_to_client(
    mut chunk_created_event_reader: EventReader<ChunkCreatedEvent>,
    mut server: ResMut<RenetServer>,
) {
    for event in chunk_created_event_reader.read() {
        for client_id in server.clients_id() {
            if let Ok(message) = bincode::serialize::<Chunk>(&event.chunk) {
                server.send_message(client_id, ServerChannel::Chunks, message)
            };
        }
    }
}
