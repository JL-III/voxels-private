use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_debug_grid::DebugGridPlugin;
use bevy_renet::{
    client_connected,
    renet::{
        transport::{ClientAuthentication, NetcodeClientTransport, NetcodeTransportError},
        ClientId, RenetClient,
    },
    RenetClientPlugin,
};
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};
use renet_visualizer::{RenetClientVisualizer, RenetVisualizerStyle};
use std::{collections::HashMap, net::UdpSocket, time::SystemTime};

use voxels::{
    app_state::plugin::AppStatePlugin, command_system::plugin::CommandPlugin,
    debug_menu::plugin::DebugPlugin, main_menu::plugin::MainMenuPlugin,
    player::client::plugin::PlayerClientPlugin, world::plugin::WorldPlugin, *,
};

#[derive(Component)]
struct ControlledPlayer;

#[derive(Default, Resource)]
struct NetworkMapping(HashMap<Entity, Entity>);

#[derive(Debug)]
struct PlayerInfo {
    client_entity: Entity,
    server_entity: Entity,
}

#[derive(Debug, Default, Resource)]
struct ClientLobby {
    players: HashMap<ClientId, PlayerInfo>,
}

#[derive(Debug, Resource)]
struct CurrentClientId(u64);

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Connected;

fn main() {
    let mut app = App::new();
    let client = RenetClient::new(connection_config());
    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let client_id = current_time.as_millis() as u64;
    let authentication = ClientAuthentication::Unsecure {
        client_id,
        protocol_id: PROTOCOL_ID,
        server_addr,
        user_data: None,
    };
    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();
    app.configure_sets(Update, Connected.run_if(client_connected()));
    app.add_plugins(RenetClientPlugin);
    app.add_plugins(FrameTimeDiagnosticsPlugin);
    app.add_plugins(LogDiagnosticsPlugin::default());
    app.add_plugins(bevy_renet::transport::NetcodeClientPlugin);
    app.add_plugins(DefaultPlugins)
        .add_plugins(ScreenDiagnosticsPlugin::default())
        .add_plugins(ScreenFrameDiagnosticsPlugin)
        .add_plugins(AppStatePlugin)
        .add_plugins(PlayerClientPlugin)
        .add_plugins(WorldPlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(MainMenuPlugin)
        .add_plugins(CommandPlugin)
        .add_plugins((DebugGridPlugin::with_floor_grid(),));
    app.insert_resource(client);
    app.insert_resource(transport);
    app.insert_resource(CurrentClientId(client_id));
    app.insert_resource(ClientLobby::default());
    app.insert_resource(NetworkMapping::default());
    app.insert_resource(RenetClientVisualizer::<200>::new(
        RenetVisualizerStyle::default(),
    ));
    app.add_systems(Update, panic_on_error_system);
    app.add_systems(Update, (client_sync_players,).in_set(Connected));
    app.run();
}

// If any error is found we just panic
fn panic_on_error_system(mut renet_error: EventReader<NetcodeTransportError>) {
    if let Some(e) = renet_error.read().next() {
        panic!("{}", e);
    }
}

fn client_sync_players(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut client: ResMut<RenetClient>,
    client_id: Res<CurrentClientId>,
    mut lobby: ResMut<ClientLobby>,
    mut network_mapping: ResMut<NetworkMapping>,
) {
    let client_id = client_id.0;
    while let Some(message) = client.receive_message(ServerChannel::ServerMessages) {
        let server_message = bincode::deserialize(&message).unwrap();
        match server_message {
            ServerMessages::PlayerCreate {
                id,
                translation,
                entity,
            } => {
                println!("Player {} connected.", id);
                let mut client_entity = commands.spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Capsule::default())),
                    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                    transform: Transform::from_xyz(translation[0], translation[1], translation[2]),
                    ..Default::default()
                });

                if client_id == id.raw() {
                    client_entity.insert(ControlledPlayer);
                }

                let player_info = PlayerInfo {
                    server_entity: entity,
                    client_entity: client_entity.id(),
                };
                lobby.players.insert(id, player_info);
                network_mapping.0.insert(entity, client_entity.id());
            }
            ServerMessages::PlayerRemove { id } => {
                println!("Player {} disconnected.", id);
                if let Some(PlayerInfo {
                    server_entity,
                    client_entity,
                }) = lobby.players.remove(&id)
                {
                    commands.entity(client_entity).despawn();
                    network_mapping.0.remove(&server_entity);
                }
            }
        }
    }
}
