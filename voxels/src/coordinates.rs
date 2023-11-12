use bevy::prelude::*;

pub struct CoordinatePlugin;

#[derive(Component)]
pub struct CoordinateDisplay {}

pub fn spawn_coordinate_display(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands
        .spawn((
            TextBundle {
                text: Text {
                    sections: vec![TextSection::new(
                        "coordinates",
                        TextStyle {
                            font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                            font_size: 32.0,
                            color: Color::WHITE,
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

impl Plugin for CoordinatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_coordinate_display);
    }
}
