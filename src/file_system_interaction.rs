use bevy::prelude::*;

pub(crate) mod asset_loading;
pub(crate) mod audio;
pub(crate) mod config;

/// Handles loading and saving of levels and save states to disk.
/// Split into the following sub-plugins:
/// - [`asset_loading::plugin`] handles loading of assets.els.
/// - [`audio::plugin`]: Handles audio initialization
pub(super) fn plugin(app: &mut App) {
    app.add_plugins((asset_loading::plugin, audio::plugin));
}
