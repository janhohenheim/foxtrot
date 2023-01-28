pub mod map;
pub mod spawning;

use crate::level_instanciation::map::MapPlugin;
use crate::level_instanciation::spawning::SpawningPlugin;
use bevy::prelude::*;

pub struct LevelInstanciationPlugin;

impl Plugin for LevelInstanciationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MapPlugin).add_plugin(SpawningPlugin);
    }
}
