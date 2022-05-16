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
        .with_input_delay(3)
        .with_max_prediction_window(12)
        .with_catchup_speed(2)
        .expect("Invalid catchup speed")
        .with_fps(FPS)
        .expect("Invalid FPS")
}
