use bevy::prelude::*;
use bevy_renet::renet::RenetClient;

use crate::{player::lib::Player, ServerChannel};

pub fn server_update_player(
    // might want this here, not sure.
    // mut player_move_event_writer: EventWriter<PlayerMoveEvent>,
    mut transform_query: Query<&mut Transform, With<Player>>,
    mut client: ResMut<RenetClient>,
) {
    for mut transform in transform_query.iter_mut() {
        while let Some(server_message) = client.receive_message(ServerChannel::PlayerSyncLocation) {
            if let Ok(server_dictate_player_position) =
                bincode::deserialize::<Vec3>(&server_message)
            {
                println!("server: {}", server_dictate_player_position);
                println!("client: {}", transform.translation);
                transform.translation = server_dictate_player_position;
                return;
            }
        }
    }
}
