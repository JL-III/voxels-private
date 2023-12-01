use bevy::prelude::*;
use bevy_renet::renet::RenetServer;

use crate::ServerChannel;

use super::events::DictatePlayerPositionEvent;

pub fn dictate_player_position(
    mut dictate_player_position_event_reader: EventReader<DictatePlayerPositionEvent>,
    mut server: ResMut<RenetServer>,
) {
    for event in dictate_player_position_event_reader.read() {
        println!("recieved event");
        for client_id in server.clients_id() {
            println!("client calculated player direction: {}", event.position);
            if let Ok(message) = bincode::serialize::<Vec3>(&event.position) {
                server.send_message(client_id, ServerChannel::PlayerSyncLocation, message)
            };
        }
    }
}
