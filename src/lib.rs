#![feature(stmt_expr_attributes)]
#![feature(let_chains)]
#![feature(fs_try_exists)]
#![feature(never_type)]

mod file_system_interaction;
mod level_design;
mod menu;
mod movement_gameplay;
mod shader;
mod util;
mod world_interaction;

use crate::file_system_interaction::asset_loading::LoadingPlugin;
use crate::file_system_interaction::game_serialization::SavingPlugin;
use crate::file_system_interaction::level_serialization::WorldSerializationPlugin;
use crate::level_design::map::MapPlugin;
use crate::level_design::spawning::SpawningPlugin;
use crate::menu::MenuPlugin;
use crate::movement_gameplay::camera::CameraPlugin;
use crate::movement_gameplay::general_movement::MovementPlugin;
use crate::movement_gameplay::navigation::NavigationPlugin;
use crate::movement_gameplay::player::PlayerPlugin;
use crate::shader::ShaderPlugin;
use crate::util::dev::DevPlugin;
use crate::world_interaction::condition::ConditionPlugin;
use crate::world_interaction::dialog::DialogPlugin;
use crate::world_interaction::interactions_ui::InteractionsUi;
use bevy::app::App;
use bevy::prelude::*;
use movement_gameplay::actions::ActionsPlugin;
use movement_gameplay::audio::InternalAudioPlugin;
use movement_gameplay::physics::PhysicsPlugin;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(PhysicsPlugin)
            .add_plugin(InteractionsUi)
            .add_plugin(DialogPlugin)
            .add_plugin(MapPlugin)
            .add_plugin(SpawningPlugin)
            .add_plugin(ConditionPlugin)
            .add_plugin(SavingPlugin)
            .add_plugin(NavigationPlugin)
            .add_plugin(ShaderPlugin)
            .add_plugin(WorldSerializationPlugin)
            .add_plugin(MovementPlugin)
            .add_plugin(DevPlugin);
    }
}
