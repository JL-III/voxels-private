use bevy::prelude::*;

#[derive(Event)]
pub struct PlayerMoveEvent {
    pub starting_position: Vec3,
    pub final_position: Vec3,
}
