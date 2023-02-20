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

/// Handles loading and saving of levels and save states to disk.
/// Split into the following sub-plugins:
/// - [`LoadingPlugin`] handles loading of assets.
/// - [`GameStateSerializationPlugin`] handles saving and loading of game states.
/// - [`LevelSerializationPlugin`] handles saving and loading of levels.
/// - [`InternalAudioPlugin`]: Handles audio initialization
pub struct FileSystemInteractionPlugin;

impl Plugin for FileSystemInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LoadingPlugin)
            .add_plugin(GameStateSerializationPlugin)
            .add_plugin(LevelSerializationPlugin)
            .add_plugin(InternalAudioPlugin);
    }
}
