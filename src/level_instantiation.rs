use crate::level_instantiation::{
    blender_workflow::blender_workflow_plugin, map::map_plugin, on_spawn::on_spawn_plugin,
};
use bevy::prelude::*;

mod blender_workflow;
mod map;
pub(crate) mod on_spawn;

/// Handles creation of levels and objects. Split into the following sub-plugins:
/// - [`map_plugin`] handles loading of level files and orchestrates the spawning of the objects therein.
/// - [`on_spawn_plugin`] handles the spawning of objects in general.
/// - [`blender_workflow`] handles the integration with [kaosat's Blender workflow](https://github.com/kaosat-dev/Blender_bevy_components_workflow)
pub(crate) fn level_instantiation_plugin(app: &mut App) {
    app.add_plugins((map_plugin, on_spawn_plugin, blender_workflow_plugin));
}
