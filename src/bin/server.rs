use bevy::input::InputPlugin;
use bevy_renet::renet::transport::{NetcodeServerTransport, ServerAuthentication, ServerConfig};

use bevy_renet::transport::NetcodeServerPlugin;
use std::{net::UdpSocket, time::SystemTime};
use voxels::app_state::state::AppState;
use voxels::command_system::events::CommandDispatchEvent;
use voxels::player::server::plugin::PlayerServerPlugin;
use voxels::{connection_config, PROTOCOL_ID};

use bevy::prelude::*;
use bevy_renet::{renet::RenetServer, RenetServerPlugin};

fn main() {
    let server_transport = build_server();
    let mut app = App::new();
    app.add_state::<AppState>();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(InputPlugin);
    app.add_plugins(RenetServerPlugin);
    app.add_event::<CommandDispatchEvent>();
    app.add_plugins(NetcodeServerPlugin);
    app.insert_resource(server_transport.0);
    app.insert_resource(server_transport.1);
    app.add_plugins(PlayerServerPlugin);

    app.run();
}

fn build_server() -> (RenetServer, NetcodeServerTransport) {
    let server = RenetServer::new(connection_config());
    let public_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind(public_addr).unwrap();
    let current_time: std::time::Duration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let server_config = ServerConfig {
        current_time,
        max_clients: 64,
        protocol_id: PROTOCOL_ID,
        public_addresses: vec![public_addr],
        authentication: ServerAuthentication::Unsecure,
    };
    let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
    (server, transport)
}
