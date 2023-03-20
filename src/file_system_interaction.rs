pub mod asset_loading;
pub mod audio;
pub mod config;
pub mod game_state_serialization;
pub mod level_serialization;

use bevy::prelude::*;

use crate::file_system_interaction::asset_loading::LoadingPlugin;
use crate::file_system_interaction::audio::InternalAudioPlugin;
use crate::file_system_interaction::game_state_serialization::GameStateSerializationPlugin;
use crate::file_system_interaction::level_serialization::LevelSerializationPlugin;
use seldom_fn_plugin::FnPluginExt;

/// Handles loading and saving of levels and save states to disk.
/// Split into the following sub-plugins:
/// - [`LoadingPlugin`] handles loading of assets.
/// - [`GameStateSerializationPlugin`] handles saving and loading of game states.
/// - [`LevelSerializationPlugin`] handles saving and loading of levels.
/// - [`InternalAudioPlugin`]: Handles audio initialization
pub fn FileSystemInteractionPlugin(app: &mut App) {
    app.fn_plugin(LoadingPlugin)
        .fn_plugin(GameStateSerializationPlugin)
        .fn_plugin(LevelSerializationPlugin)
        .fn_plugin(InternalAudioPlugin);
}
