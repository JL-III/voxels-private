use bevy::prelude::*;

#[derive(Event)]
pub struct CommandDispatchEvent {
    pub command: String,
}
