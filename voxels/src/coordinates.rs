use bevy::prelude::*;

use crate::camera::FlyCam;

pub struct CoordinatePlugin;

#[derive(Component)]
pub struct CoordinateDisplay {}

pub fn spawn_coordinate_display(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    flycam_query: Query<&mut Transform, With<FlyCam>>,
) {
    let _main_menu_entity = build_coordinate_display(&mut commands, asset_server, flycam_query);
}

pub fn build_coordinate_display(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    flycam_query: Query<&mut Transform, With<FlyCam>>,
) -> Entity {
    if let Ok(flycam) = flycam_query.get_single() {
        let coordinate_display_entity = commands
            .spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            flycam.translation.to_string(),
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 16.0,
                                color: Color::WHITE,
                            },
                        )],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                },
                CoordinateDisplay {},
            ))
            .id();
        coordinate_display_entity
    } else {
        let coordinate_display_entity = commands
            .spawn((
                TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            "coordinates",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 16.0,
                                color: Color::WHITE,
                            },
                        )],
                        alignment: TextAlignment::Center,
                        ..default()
                    },
                    ..default()
                },
                CoordinateDisplay {},
            ))
            .id();
        coordinate_display_entity
    }
}

impl Plugin for CoordinatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_coordinate_display);
    }
}
