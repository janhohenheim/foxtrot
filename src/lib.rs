#![feature(stmt_expr_attributes)]
#![feature(let_chains)]
#![feature(fs_try_exists)]
#![feature(never_type)]

mod actions;
mod audio;
mod condition;
mod dev;
mod dialog;
mod file_system_interaction;
mod interactions_ui;
mod map;
mod math;
mod menu;
mod movement_gameplay;
mod physics;
#[cfg(feature = "editor")]
mod scene_editor;
mod shader;
mod spawning;
mod trait_extension;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::condition::ConditionPlugin;
use crate::dev::DevPlugin;
use crate::dialog::DialogPlugin;
use crate::file_system_interaction::asset_loading::LoadingPlugin;
use crate::file_system_interaction::game_serialization::SavingPlugin;
use crate::file_system_interaction::level_serialization::WorldSerializationPlugin;
use crate::interactions_ui::InteractionsUi;
use crate::map::MapPlugin;
use crate::menu::MenuPlugin;
use crate::movement_gameplay::camera::CameraPlugin;
use crate::movement_gameplay::general_movement::MovementPlugin;
use crate::movement_gameplay::navigation::NavigationPlugin;
use crate::movement_gameplay::player::PlayerPlugin;
use crate::physics::PhysicsPlugin;
use crate::shader::ShaderPlugin;
use crate::spawning::SpawningPlugin;
use bevy::app::App;
use bevy::prelude::*;

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
