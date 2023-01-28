pub mod asset_loading;
pub mod game_serialization;
pub mod level_serialization;

use bevy::prelude::*;

use crate::file_system_interaction::asset_loading::LoadingPlugin;
use crate::file_system_interaction::game_serialization::SavingPlugin;
use crate::file_system_interaction::level_serialization::WorldSerializationPlugin;

pub struct FileSystemInteractionPlugin;

impl Plugin for FileSystemInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LoadingPlugin)
            .add_plugin(SavingPlugin)
            .add_plugin(WorldSerializationPlugin);
    }
}
