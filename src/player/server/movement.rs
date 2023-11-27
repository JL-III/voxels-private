use bevy::prelude::*;
use bevy_renet::renet::RenetServer;

use crate::{
    player::lib::{MovementSettings, Player},
    ClientChannel, PlayerInput,
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
            if let Ok(player_move) = bincode::deserialize::<PlayerInput>(&message) {
                for mut transform in transform_query.iter_mut() {
                    // let mut player_move_event = PlayerMoveEvent {
                    //   starting_position: transform.translation,
                    //   final_position: transform.translation,
                    // };
                    let mut velocity = Vec3::ZERO;
                    let local_z = transform.local_z();
                    let forward = Vec3::new(local_z.x, 0., local_z.z);
                    let right = Vec3::new(local_z.z, 0., -local_z.x);
                    if player_move.up {
                        velocity += Vec3::Y
                    }
                    if player_move.down {
                        velocity -= Vec3::Y
                    }
                    if player_move.forward {
                        velocity += forward
                    }
                    if player_move.backward {
                        velocity -= forward
                    }
                    if player_move.right {
                        velocity -= right
                    }
                    if player_move.left {
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
