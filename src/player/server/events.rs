use bevy::prelude::*;

use crate::player::{
    client::events::PlayerMoveEvent,
    lib::{MovementSettings, Player},
};

#[derive(Event)]
pub struct ClientSentMoveEvent {
    pub direction: Vec3,
}

#[derive(Event)]
pub struct DictatePlayerPositionEvent {
    pub position: Vec3,
}

#[derive(Resource)]
pub struct PlayerSyncLocationTimer(pub Timer);

pub fn client_sent_move_event_handler(
    time: Res<Time>,
    settings: Res<MovementSettings>,
    mut transform_query: Query<&mut Transform, With<Player>>,
    mut client_sent_move_event_reader: EventReader<ClientSentMoveEvent>,
    mut player_move_event_writer: EventWriter<PlayerMoveEvent>,
    mut dictate_player_position_event_writer: EventWriter<DictatePlayerPositionEvent>,
    mut timer: ResMut<PlayerSyncLocationTimer>,
) {
    for event in client_sent_move_event_reader.read() {
        for mut transform in transform_query.iter_mut() {
            let mut player_move_event = PlayerMoveEvent {
                starting_position: transform.translation,
                final_position: transform.translation,
            };

            let velocity = event.direction.normalize_or_zero();

            transform.translation += velocity * time.delta_seconds() * settings.speed;
            timer.0.tick(time.delta());

            if timer.0.finished() {
                println!("server position: {}", transform.translation);
                let dictate_position = DictatePlayerPositionEvent {
                    position: transform.translation,
                };
                dictate_player_position_event_writer.send(dictate_position);
            }
            player_move_event.final_position = transform.translation;
            player_move_event_writer.send(PlayerMoveEvent {
                starting_position: player_move_event.starting_position,
                final_position: player_move_event.final_position,
            })
        }
    }
}
