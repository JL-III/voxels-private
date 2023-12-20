use bevy::prelude::*;
use bevy_renet::renet::RenetClient;

use crate::net::ClientChannel;

use super::events::PlayerMoveEvent;

pub fn send_player_movement(
    mut player_move_event_reader: EventReader<PlayerMoveEvent>,
    mut client: ResMut<RenetClient>,
) {
    for event in player_move_event_reader.read() {
        let player_input = get_player_move_direction(event);
        if let Ok(player_input_message) = bincode::serialize(&player_input) {
            // temporarily disabled to experiment with physics
            client.send_message(ClientChannel::Input, player_input_message);
        } else {
            warn!("could not serialize player_input_message");
        }
    }
}

pub fn get_player_move_direction(player_move_event: &PlayerMoveEvent) -> Vec3 {
    let direction = player_move_event.final_position - player_move_event.starting_position;
    direction.normalize()
}
