use bevy::prelude::*;

use crate::AppState;

pub fn transition_to_command_state(
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Slash) {
        match *app_state.get() {
            AppState::Game => {
                next_app_state.set(AppState::Command);
                println!("Entered AppState::Command");
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
                println!("Entered AppState::Game");
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
        println!("Entered AppState::Paused");
    }
}
