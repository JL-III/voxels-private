use bevy::prelude::*;
use bevy_renet::renet::RenetClient;

use crate::{world::chunk::Chunk, net::ServerChannel};

use super::events::RenderChunk;

pub fn get_chunk_from_server(
    mut client: ResMut<RenetClient>,
    mut render_chunk_writer: EventWriter<RenderChunk>,
) {
    while let Some(server_message) = client.receive_message(ServerChannel::Chunks) {
        if let Ok(chunk) = bincode::deserialize::<Chunk>(&server_message) {
            render_chunk_writer.send(RenderChunk { chunk });
            println!("chunk recieved!");
        }
    }
}
