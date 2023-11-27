use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Component, Event)]
pub struct PlayerMoveEvent {
    pub starting_position: Vec3,
    pub final_position: Vec3,
}

#[derive(Event)]
pub struct PlayerSpawnEvent {
    pub position: Vec3,
    pub entity_id: Entity,
}
