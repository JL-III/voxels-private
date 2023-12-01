use std::time::Duration;

use bevy::{
    app::{App, FixedUpdate, Plugin, Startup, Update},
    ecs::event::Event,
    math::Vec3,
    time::{Timer, TimerMode},
};
use bevy_renet::renet::RenetClient;

use crate::{
    connection_config,
    player::{
        events::{PlayerMoveEvent, PlayerSpawnEvent},
        lib::{InputState, MovementSettings},
    },
};

use super::{
    event_handlers::{client_sent_move_event_handler, PlayerSyncLocationTimer},
    server_in::client_move_player,
    server_out::dictate_player_position,
    setup::setup_server_player,
};

#[derive(Event)]
pub struct ClientSentMoveEvent {
    pub direction: Vec3,
}

#[derive(Event)]
pub struct DictatePlayerPositionEvent {
    pub position: Vec3,
}

pub struct PlayerServerPlugin;

impl Plugin for PlayerServerPlugin {
    fn build(&self, app: &mut App) {
        let client = RenetClient::new(connection_config());

        app.init_resource::<InputState>()
            .add_event::<ClientSentMoveEvent>()
            .add_event::<DictatePlayerPositionEvent>()
            .init_resource::<MovementSettings>()
            .insert_resource(PlayerSyncLocationTimer(Timer::new(
                Duration::from_secs(3),
                TimerMode::Repeating,
            )))
            .insert_resource(client)
            .add_event::<PlayerMoveEvent>()
            .add_event::<PlayerSpawnEvent>()
            .add_systems(Startup, setup_server_player)
            // something happening here with updates and fixed updates causing systems to miss events that are fired
            // this combination appears to work but it definitely means i am not understanding something
            .add_systems(FixedUpdate, client_sent_move_event_handler)
            .add_systems(FixedUpdate, client_move_player)
            .add_systems(Update, dictate_player_position);
    }
}
