use bevy::prelude::*;

use super::{
    state::AppState,
    transition::{
        transition_to_command_state, transition_to_game_state, transition_to_pause_state,
    },
};

pub struct AppStatePlugin;

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>()
            .add_systems(
                Update,
                transition_to_game_state.run_if(in_state(AppState::Paused)),
            )
            .add_systems(
                Update,
                transition_to_game_state.run_if(in_state(AppState::Command)),
            )
            .add_systems(
                Update,
                transition_to_pause_state.run_if(in_state(AppState::Game)),
            )
            .add_systems(
                Update,
                transition_to_command_state.run_if(in_state(AppState::Game)),
            );
    }
}
