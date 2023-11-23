use bevy::{
    app::{App, Plugin, Update},
    ecs::schedule::{common_conditions::in_state, IntoSystemConfigs, OnEnter, OnExit},
};

use crate::AppState;

use super::{
    command_interface::{
        despawn_command_interface, spawn_command_interface, update_command_interface,
    },
    events::CommandDispatchEvent,
};

pub struct CommandPlugin;

impl Plugin for CommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CommandDispatchEvent>()
            .add_systems(OnEnter(AppState::Command), spawn_command_interface)
            .add_systems(OnExit(AppState::Command), despawn_command_interface)
            // .add_systems(Update, command_listener)
            .add_systems(
                Update,
                update_command_interface.run_if(in_state(AppState::Command)),
            );
    }
}
