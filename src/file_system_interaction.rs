pub(crate) mod asset_loading;
pub(crate) mod audio;
pub(crate) mod config;

use bevy::prelude::*;

use crate::file_system_interaction::asset_loading::loading_plugin;
use crate::file_system_interaction::audio::internal_audio_plugin;
use seldom_fn_plugin::FnPluginExt;

/// Handles loading and saving of levels and save states to disk.
/// Split into the following sub-plugins:
/// - [`loading_plugin`] handles loading of assets.els.
/// - [`internal_audio_plugin`]: Handles audio initialization
pub(crate) fn file_system_interaction_plugin(app: &mut App) {
    app.fn_plugin(loading_plugin)
        .fn_plugin(internal_audio_plugin);
}
