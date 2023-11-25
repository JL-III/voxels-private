use bevy::prelude::*;

use crate::{player::events::PlayerMoveEvent, world::events::ChunkCreatedEvent};

#[derive(Component)]
pub struct CoordinateDisplay {}

#[derive(Component)]
pub struct ChunkRegistryDisplay {}

pub fn get_font(asset_server: Res<AssetServer>) -> Handle<Font> {
    asset_server.load("fonts/FiraSans-Bold.ttf")
}

pub fn spawn_coordinate_display(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle {
            text: Text {
                sections: vec![TextSection::new(
                    "coordinates",
                    TextStyle {
                        font: get_font(asset_server),
                        font_size: 32.0,
                        color: Color::rgb(3.0 / 255.0, 252.0 / 255.0, 169.0 / 255.0),
                    },
                )],
                alignment: TextAlignment::Center,
                ..default()
            },
            ..default()
        },
        CoordinateDisplay {},
    ));
}

pub fn spawn_chunk_registry_display(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                right: Val::Px(10.0),
                ..default()
            },
            text: Text {
                sections: vec![TextSection::new(
                    "chunk registry",
                    TextStyle {
                        font: get_font(asset_server),
                        font_size: 32.0,
                        color: Color::rgb(3.0 / 255.0, 252.0 / 255.0, 169.0 / 255.0),
                    },
                )],
                alignment: TextAlignment::Center,
                ..default()
            },
            ..default()
        },
        ChunkRegistryDisplay {},
    ));
}

pub fn chunk_created_listener(
    mut chunk_created_event_reader: EventReader<ChunkCreatedEvent>,
    mut chunk_registry_display_query: Query<&mut Text, With<ChunkRegistryDisplay>>,
) {
    for event in chunk_created_event_reader.read() {
        for mut text in &mut chunk_registry_display_query {
            text.sections[0].value = format!("chunk registry count: {}", event.registry_size)
        }
    }
}

pub fn player_move_event_listener(
    mut player_move_event_reader: EventReader<PlayerMoveEvent>,
    mut coordinate_display_query: Query<&mut Text, With<CoordinateDisplay>>,
) {
    for event in player_move_event_reader.read() {
        for mut text in &mut coordinate_display_query {
            text.sections[0].value = format!(
                "x: {:.4}, y: {:.4}, z: {:.4} chunk x:{} y:{} z:{}",
                event.final_position.x,
                event.final_position.y,
                event.final_position.z,
                (event.final_position.x / 16.0).floor() as isize,
                (event.final_position.y / 16.0).floor() as isize,
                (event.final_position.z / 16.0).floor() as isize,
            );
        }
    }
}
