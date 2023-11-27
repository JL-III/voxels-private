use bevy::{
    app::{App, FixedUpdate, Plugin, Startup, Update},
    ecs::schedule::{common_conditions::in_state, IntoSystemConfigs, OnEnter, OnExit},
};
use bevy_renet::renet::RenetClient;

use crate::{
    app_state::state::AppState,
    connection_config,
    player::{
        events::{PlayerMoveEvent, PlayerSpawnEvent},
        lib::{speed_command, teleport_command, InputState, MovementSettings},
    },
};

use super::{
    controls::{client_player_move, grab_cursor, initial_grab_cursor, player_look, release_cursor},
    setup::setup_client_player,
};

pub struct PlayerClientPlugin;

impl Plugin for PlayerClientPlugin {
    fn build(&self, app: &mut App) {
        let client = RenetClient::new(connection_config());

        app.init_resource::<InputState>()
            .init_resource::<MovementSettings>()
            .insert_resource(client)
            .add_event::<PlayerMoveEvent>()
            .add_event::<PlayerSpawnEvent>()
            .add_systems(Update, speed_command)
            .add_systems(Update, teleport_command)
            .add_systems(Startup, setup_client_player)
            .add_systems(Startup, initial_grab_cursor)
            .add_systems(
                FixedUpdate,
                (client_player_move, player_look).run_if(in_state(AppState::Game)),
            )
            .add_systems(OnEnter(AppState::Game), grab_cursor)
            .add_systems(OnExit(AppState::Game), release_cursor);
    }
}
