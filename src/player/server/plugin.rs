use bevy::{
    app::{App, FixedUpdate, Plugin, Startup, Update},
    ecs::schedule::{common_conditions::in_state, IntoSystemConfigs},
};
use bevy_renet::renet::RenetClient;

use crate::{
    app_state::state::AppState,
    connection_config,
    player::{
        events::{PlayerMoveEvent, PlayerSpawnEvent},
        lib::{
            player_move, setup_player, speed_command, teleport_command, InputState,
            MovementSettings,
        },
    },
};

pub struct PlayerServerPlugin;

impl Plugin for PlayerServerPlugin {
    fn build(&self, app: &mut App) {
        let client = RenetClient::new(connection_config());

        app.init_resource::<InputState>()
            .init_resource::<MovementSettings>()
            .insert_resource(client)
            .add_event::<PlayerMoveEvent>()
            .add_event::<PlayerSpawnEvent>()
            .add_systems(Update, speed_command)
            .add_systems(Update, teleport_command)
            .add_systems(Startup, setup_player)
            .add_systems(FixedUpdate, (player_move).run_if(in_state(AppState::Game)));
    }
}
