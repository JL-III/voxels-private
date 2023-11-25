use bevy::prelude::*;

use crate::player::events::PlayerMoveEvent;

#[derive(Component)]
pub struct CoordinateDisplay {}

pub fn get_font(asset_server: Res<AssetServer>) -> Handle<Font> {
    asset_server.load("fonts/FiraSans-Bold.ttf")
}

pub fn get_text_bundle(asset_server: Res<AssetServer>) -> TextBundle {
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
    }
}

pub fn spawn_coordinate_display(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((get_text_bundle(asset_server), CoordinateDisplay {}));
}

pub fn player_move_event_listener(
    mut player_move_event_reader: EventReader<PlayerMoveEvent>,
    mut coordinate_display_query: Query<&mut Text, With<CoordinateDisplay>>,
) {
    for event in player_move_event_reader.read() {
        for mut text in &mut coordinate_display_query {
            text.sections[0].value = format!(
                "x: {:.4}, y: {:.4}, z: {:.4} chunk x:{} z:{}",
                event.final_position.x,
                event.final_position.y,
                event.final_position.z,
                (event.final_position.x / 16.0).floor() as isize,
                (event.final_position.z / 16.0).floor() as isize,
            );
        }
    }
}
