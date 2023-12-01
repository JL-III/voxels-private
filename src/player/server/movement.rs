use bevy::prelude::*;
use bevy_renet::renet::RenetServer;

use crate::{
    player::lib::{MovementSettings, Player},
    ClientChannel, PlayerDirection, ServerChannel,
};

#[derive(Resource)]
pub struct PlayerSyncLocationTimer(pub Timer);

pub fn server_player_move(
    time: Res<Time>,
    settings: Res<MovementSettings>,
    mut transform_query: Query<&mut Transform, With<Player>>,
    mut server: ResMut<RenetServer>,
    mut timer: ResMut<PlayerSyncLocationTimer>,
) {
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, ClientChannel::Input) {
            if let Ok(player_move) = bincode::deserialize::<PlayerDirection>(&message) {
                for mut transform in transform_query.iter_mut() {
                    // println!("data: {:?}", player_move);
                    let velocity = player_move.0.normalize_or_zero();

                    transform.translation += velocity * time.delta_seconds() * settings.speed;
                    timer.0.tick(time.delta());

                    if timer.0.finished() {
                        println!(
                            "server calculated player position: {}",
                            transform.translation
                        );
                        println!(
                            "client calculated player direction: {}",
                            player_move.0
                        );
                        if let Ok(message) = bincode::serialize::<Vec3>(&transform.translation) {
                            server.send_message(client_id, ServerChannel::PlayerSyncLocation, message)
                        };
                    }
                }
            };
        }
    }
}
