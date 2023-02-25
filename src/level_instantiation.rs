pub mod grass;
pub mod map;
pub mod spawning;

use crate::level_instantiation::grass::GrassPlugin;
use crate::level_instantiation::map::MapPlugin;
use crate::level_instantiation::spawning::SpawningPlugin;
use bevy::prelude::*;

/// Handles creation of levels and objects. Split into the following sub-plugins:
/// - [`MapPlugin`] handles loading of level files and orchestrates the spawning of the objects therein.
/// - [`SpawningPlugin`] handles the spawning of objects in general.
/// - [`GrassPlugin`] handles the spawning of grass on top of marked meshes.
pub struct LevelInstantiationPlugin;

impl Plugin for LevelInstantiationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MapPlugin)
            .add_plugin(SpawningPlugin)
            .add_plugin(GrassPlugin);
    }
}
