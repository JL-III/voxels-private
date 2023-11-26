use bevy::{
    app::{App, FixedUpdate, Plugin, Startup, Update},
    ecs::schedule::{common_conditions::in_state, IntoSystemConfigs, OnEnter, OnExit},
};

use crate::app_state::state::AppState;

use super::{
    controls::{
        grab_cursor, initial_grab_cursor, player_look, player_move, release_cursor, setup_player,
        speed_command, teleport_command, InputState, MovementSettings,
    },
    events::{PlayerMoveEvent, PlayerSpawnEvent},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputState>()
            .init_resource::<MovementSettings>()
            .add_event::<PlayerMoveEvent>()
            .add_event::<PlayerSpawnEvent>()
            .add_systems(Update, speed_command)
            .add_systems(Update, teleport_command)
            .add_systems(Startup, setup_player)
            .add_systems(Startup, initial_grab_cursor)
            .add_systems(
                FixedUpdate,
                (player_move, player_look).run_if(in_state(AppState::Game)),
            )
            .add_systems(OnEnter(AppState::Game), grab_cursor)
            .add_systems(OnExit(AppState::Game), release_cursor);
    }
}
