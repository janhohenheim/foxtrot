use crate::config::FPS;
use ggrs::{Config, SessionBuilder};

use crate::networking::protocol::InputProtocol;

pub type Input = InputProtocol;
pub type State = u8;

pub fn create_session_builder<GGRSConfig: Config>(
    num_players: usize,
) -> SessionBuilder<GGRSConfig> {
    SessionBuilder::<GGRSConfig>::new()
        .with_num_players(num_players)
        .with_max_prediction_window(20)
        .with_fps(FPS)
        .expect("Invalid FPS")
        .with_input_delay(6)
}
