use std::net::SocketAddr;

use crate::{networking::protocol::LocalHandles, GameState};

use super::shared::{self, create_session_builder};
use bevy::{log, prelude::*};
use bevy_ggrs::SessionType;
use clap::Parser;
use ggrs::{Config, PlayerType, SessionBuilder, UdpNonBlockingSocket};

#[derive(Debug, Default)]
pub struct NativePlugin;
impl Plugin for NativePlugin {
    fn build(&self, app: &mut App) {
        log::info!("Using native networking plugin");
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(start_session));
    }
}

/// You need to define a config struct to bundle all the generics of GGRS. You can safely ignore `State` and leave it as u8 for all GGRS functionality.
/// Source: https://github.com/gschup/bevy_ggrs/blob/7d3def38720161610313c7031d6f1cb249098b43/examples/box_game/box_game.rs#L27
#[derive(Debug)]
pub struct NativeConfig;
impl Config for NativeConfig {
    type Input = shared::Input;
    type State = shared::State;
    type Address = SocketAddr;
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    local_port: u16,
    #[clap(short, long)]
    players: Vec<String>,
}

fn start_session(mut commands: Commands) {
    let args = Args::parse();
    let num_players = args.players.len();
    log::info!("Got args: {:?}", args);
    assert!(num_players == 2);

    // create a GGRS session
    let mut p2p_session: SessionBuilder<NativeConfig> = create_session_builder(num_players);
    let mut handles = Vec::new();
    // add players
    for (i, player_addr) in args.players.into_iter().enumerate() {
        // local player
        if player_addr == "localhost" {
            p2p_session = p2p_session
                .add_player(PlayerType::Local, i)
                .expect("Failed to add local player");
            handles.push(i);
        } else {
            // remote players
            let remote_addr: SocketAddr = player_addr.parse().unwrap();
            p2p_session = p2p_session
                .add_player(PlayerType::Remote(remote_addr), i)
                .expect("Failed to add remote player");
        }
    }

    // start the GGRS session
    let socket = UdpNonBlockingSocket::bind_to_port(args.local_port).unwrap();
    let session = p2p_session.start_p2p_session(socket).unwrap();

    commands.insert_resource(session);
    commands.insert_resource(LocalHandles { handles });
    commands.insert_resource(SessionType::P2PSession);
}
