use bevy::prelude::*;

pub struct CoordinatePlugin;

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

impl Plugin for CoordinatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_coordinate_display);
    }
}

pub fn unwrap() {
    let some_option = Some("Rust");
    let value = some_option.unwrap(); // This line should trigger the clippy::unwrap_used lint
    println!("Value: {}", value);
}