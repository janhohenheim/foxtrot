pub mod map;
#[cfg(feature = "editor")]
pub mod scene_editor;
pub mod spawning;

use crate::level_design::map::MapPlugin;
use crate::level_design::spawning::SpawningPlugin;
use bevy::prelude::*;

pub struct LevelDesignPlugin;

impl Plugin for LevelDesignPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MapPlugin).add_plugin(SpawningPlugin);
    }
}
