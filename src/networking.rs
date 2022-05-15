use crate::actions::{create_input_protocol, set_movement_actions, Actions};
use crate::config::FPS;
use crate::player::move_players;
use crate::GameState;
use bevy::prelude::*;
use bevy_ggrs::GGRSPlugin;
mod targets;
use targets::{PlatformConfig, PlatformPlugin};
pub mod protocol;

pub struct NetworkingPlugin;
const ROLLBACK_SYSTEMS: &str = "rollback_systems";

#[derive(SystemLabel, Debug, Clone, Hash, Eq, PartialEq)]
enum Systems {
    Input,
    Move,
}

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut App) {
        GGRSPlugin::<PlatformConfig>::new()
            .with_input_system(create_input_protocol)
            .with_update_frequency(FPS)
            .with_rollback_schedule(
                Schedule::default().with_stage(
                    ROLLBACK_SYSTEMS,
                    SystemStage::parallel()
                        .with_system_set(State::<GameState>::get_driver())
                        .with_system_set(
                            SystemSet::on_update(GameState::Playing)
                                .with_system(set_movement_actions.label(Systems::Input))
                                .with_system(
                                    move_players.label(Systems::Move).after(Systems::Input),
                                ),
                        ),
                ),
            )
            .register_rollback_type::<Transform>()
            .register_rollback_type::<Actions>()
            .build(app);

        app.add_plugin(PlatformPlugin::default());
    }
}
