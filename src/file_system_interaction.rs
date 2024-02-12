use crate::file_system_interaction::{asset_loading::loading_plugin, audio::internal_audio_plugin};
use bevy::prelude::*;
use seldom_fn_plugin::FnPluginExt;

pub(crate) mod asset_loading;
pub(crate) mod audio;
pub(crate) mod config;

/// Handles loading and saving of levels and save states to disk.
/// Split into the following sub-plugins:
/// - [`loading_plugin`] handles loading of assets.els.
/// - [`internal_audio_plugin`]: Handles audio initialization
pub(crate) fn file_system_interaction_plugin(app: &mut App) {
    app.fn_plugin(loading_plugin)
        .fn_plugin(internal_audio_plugin);
}
