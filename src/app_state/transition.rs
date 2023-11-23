use bevy::prelude::*;

use super::state::AppState;

pub fn transition_to_command_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    // transition to command state can only occur when someone is playing the game
    if *app_state.get() != AppState::Game {
        return;
    };
    if let Some(key) = keyboard_input.get_just_pressed().next() {
        match key {
            KeyCode::Slash | KeyCode::T => {
                next_app_state.set(AppState::Command);
            }
            _ => {}
        }
    }
}

pub fn transition_to_game_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match *app_state.get() {
            AppState::Paused => {
                next_app_state.set(AppState::Game);
            }
            AppState::Command => {
                next_app_state.set(AppState::Game);
            }
            _ => {}
        }
    }
}

pub fn transition_to_pause_state(
    keyboard_input: Res<Input<KeyCode>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        next_app_state.set(AppState::Paused);
    }
}
