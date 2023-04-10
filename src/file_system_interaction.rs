pub(crate) mod asset_loading;
pub(crate) mod audio;
pub(crate) mod config;
pub(crate) mod game_state_serialization;
pub(crate) mod level_serialization;

use bevy::prelude::*;

use crate::file_system_interaction::asset_loading::loading_plugin;
use crate::file_system_interaction::audio::internal_audio_plugin;
use crate::file_system_interaction::game_state_serialization::game_state_serialization_plugin;
use crate::file_system_interaction::level_serialization::level_serialization_plugin;
use seldom_fn_plugin::FnPluginExt;

/// Handles loading and saving of levels and save states to disk.
/// Split into the following sub-plugins:
/// - [`loading_plugin`] handles loading of assets.
/// - [`game_state_serialization_plugin`] handles saving and loading of game states.
/// - [`level_serialization_plugin`] handles saving and loading of levels.
/// - [`internal_audio_plugin`]: Handles audio initialization
pub(crate) fn file_system_interaction_plugin(app: &mut App) {
    app.fn_plugin(loading_plugin)
        .fn_plugin(game_state_serialization_plugin)
        .fn_plugin(level_serialization_plugin)
        .fn_plugin(internal_audio_plugin);
}
