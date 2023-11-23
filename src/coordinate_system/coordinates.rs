use bevy::prelude::*;

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
