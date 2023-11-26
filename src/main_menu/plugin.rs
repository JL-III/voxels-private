use bevy::prelude::*;

use crate::app_state::state::AppState;

use super::{
    interactions::{interact_with_play_button, interact_with_quit_button},
    layout::{despawn_main_menu, spawn_main_menu},
};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Paused), spawn_main_menu)
            .add_systems(OnExit(AppState::Paused), despawn_main_menu)
            .add_systems(
                Update,
                (interact_with_play_button, interact_with_quit_button)
                    .run_if(in_state(AppState::Paused)),
            );
    }
}
