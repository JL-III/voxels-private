use bevy::prelude::*;
use bevy_renet::renet::RenetServer;

use crate::net::{ClientChannel, PlayerDirection};

use super::events::ClientSentMoveEvent;

///
/// listens for a client message for player movement to come through
/// fires off the server side ClientSentMoveEvent
///
pub fn client_move_player(
    mut server: ResMut<RenetServer>,
    mut server_event_writer: EventWriter<ClientSentMoveEvent>,
) {
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, ClientChannel::Input) {
            if let Ok(player_move) = bincode::deserialize::<PlayerDirection>(&message) {
                //this is where the event is fired
                server_event_writer.send(ClientSentMoveEvent {
                    direction: player_move.0,
                });
            };
        }
    }
}
