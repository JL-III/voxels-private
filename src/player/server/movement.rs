use bevy::prelude::*;
use bevy_renet::renet::RenetServer;

use crate::{
    player::lib::{MovementSettings, Player},
    ClientChannel, PlayerMovement,
};

pub fn server_player_move(
    // mut player_move_event_writer: EventWriter<PlayerMoveEvent>,
    time: Res<Time>,
    settings: Res<MovementSettings>,
    mut transform_query: Query<&mut Transform, With<Player>>,
    mut server: ResMut<RenetServer>,
) {
    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, ClientChannel::Input) {
            if let Ok(player_move) = bincode::deserialize::<PlayerMovement>(&message) {
                for mut transform in transform_query.iter_mut() {
                    println!("data: {:?}", player_move);
                    let mut velocity = Vec3::ZERO;
                    let local_z = transform.local_z();
                    let forward = Vec3::new(local_z.x, 0., local_z.z);
                    let right = Vec3::new(local_z.z, 0., -local_z.x);
                    if player_move.input.up {
                        velocity += Vec3::Y
                    }
                    if player_move.input.down {
                        velocity -= Vec3::Y
                    }
                    if player_move.input.forward {
                        velocity += forward
                    }
                    if player_move.input.backward {
                        velocity -= forward
                    }
                    if player_move.input.right {
                        velocity -= right
                    }
                    if player_move.input.left {
                        velocity += right
                    }
                    velocity = velocity.normalize_or_zero();

                    transform.translation += velocity * time.delta_seconds() * settings.speed;
                    println!("new player position: {}", transform.translation);
                }
            };
        }
    }
}