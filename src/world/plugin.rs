use bevy::prelude::*;
use bevy::utils::Duration;
use bevy_atmosphere::prelude::*;

use super::{
    atmosphere::{daylight_cycle, setup_environment, CycleTimer},
    block::{spawn_cube, VertexScale},
    chunk::{
        change_vertex_scale_command, chunk_enter_listener, player_move_event_listener, render,
        Chunk, ChunkRegistry, despawn_chunks_command,
    },
    events::{ChunkCreatedEvent, ChunkEnterEvent},
};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ChunkRegistry {
            chunks: Vec::<Chunk>::new(),
        })
        .insert_resource(VertexScale { scale: 1.0 })
        .insert_resource(Msaa::Sample4)
        .insert_resource(AtmosphereModel::new(Nishita {
            sun_position: Vec3::new(-1., 0., 0.),
            rayleigh_coefficient: Vec3::new(1e-5, 1e-5, 1e-5),
            ..default()
        })) // Default Atmosphere material, we can edit it to simulate another planet
        .insert_resource(CycleTimer(Timer::new(
            Duration::from_millis(50),
            TimerMode::Repeating,
        )))
        .add_plugins((AtmospherePlugin,))
        .add_event::<ChunkCreatedEvent>()
        .add_event::<ChunkEnterEvent>()
        .add_systems(Update, spawn_cube)
        .add_systems(Update, chunk_enter_listener)
        .add_systems(Update, change_vertex_scale_command)
        .add_systems(Update, render)
        .add_systems(Update, player_move_event_listener)
        .add_systems(Startup, setup_environment)
        .add_systems(Update, daylight_cycle)
        .add_systems(Update, despawn_chunks_command);
    }
}
