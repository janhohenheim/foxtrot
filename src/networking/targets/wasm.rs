use super::shared::{self, create_session_builder};
use crate::{networking::protocol::LocalHandles, GameState};
use bevy::{log, prelude::*, tasks::IoTaskPool};
use bevy_ggrs::SessionType;
use bevy_web_resizer::Plugin as WebResizerPlugin;
use ggrs::{Config, PlayerType, SessionBuilder};
use matchbox_socket::WebRtcSocket;

#[derive(Debug, Default)]
pub struct WasmPlugin;
impl Plugin for WasmPlugin {
    fn build(&self, app: &mut App) {
        log::info!("Using wasm networking plugin");

        app.add_system_set(
            SystemSet::on_enter(GameState::Playing).with_system(start_matchbox_socket),
        )
        .add_system_set(SystemSet::on_update(GameState::Playing).with_system(wait_for_players));

        app.add_plugin(WebResizerPlugin);
    }
}

/// You need to define a config struct to bundle all the generics of GGRS. You can safely ignore `State` and leave it as u8 for all GGRS functionality.
/// Source: https://github.com/gschup/bevy_ggrs/blob/7d3def38720161610313c7031d6f1cb249098b43/examples/box_game/box_game.rs#L27
#[derive(Debug)]
pub struct WasmConfig;
impl Config for WasmConfig {
    type Input = shared::Input;
    type State = shared::State;
    type Address = String;
}

fn start_matchbox_socket(mut commands: Commands, task_pool: Res<IoTaskPool>) {
    let room_url = "wss://matchbox.hohenheim.ch/extreme-bevy/next_2";
    log::info!("Connecting to matchbox server: {}", room_url);
    let (socket, message_loop) = WebRtcSocket::new(room_url);

    // The message loop needs to be awaited, or nothing will happen.
    // We do this here using bevy's task system.
    task_pool.spawn(message_loop).detach();
    commands.insert_resource(Some(socket));
}

fn wait_for_players(mut commands: Commands, mut socket: ResMut<Option<WebRtcSocket>>) {
    let socket = socket.as_mut();
    if socket.is_none() {
        // If there is no socket we've already started the game
        return;
    }
    // Check for new connections
    socket.as_mut().unwrap().accept_new_connections();
    let players = socket.as_ref().unwrap().players();

    let num_players = 2;
    if players.len() < num_players {
        return;
    }

    log::info!("All players have joined, starting game");

    // consume the socket (currently required because GGRS takes ownership of its socket)
    let socket = socket.take().unwrap();

    // create a GGRS P2P session
    let mut p2p_session: SessionBuilder<WasmConfig> = create_session_builder(num_players);

    let mut handles = Vec::new();
    for (i, player_type) in players.into_iter().enumerate() {
        if player_type == PlayerType::Local {
            handles.push(i);
        }
        p2p_session = p2p_session
            .add_player(player_type, i)
            .expect("Failed to add player");
    }

    // start the GGRS session
    let session = p2p_session
        .start_p2p_session(socket)
        .expect("Session could not be created.");
    commands.insert_resource(session);
    commands.insert_resource(LocalHandles { handles });
    commands.insert_resource(SessionType::P2PSession);
}
