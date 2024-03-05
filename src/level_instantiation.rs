use bevy::prelude::*;

mod blender_workflow;
mod map;
pub(crate) mod on_spawn;

/// Handles creation of levels and objects. Split into the following sub-plugins:
/// - [`map::plugin`] handles loading of level files and orchestrates the spawning of the objects therein.
/// - [`on_spawn::plugin`] handles the spawning of objects in general.
/// - [`blender_workflow::plugin`] handles the integration with [kaosat's Blender workflow](https://github.com/kaosat-dev/Blender_bevy_components_workflow)
pub(super) fn plugin(app: &mut App) {
    app.add_plugins((map::plugin, on_spawn::plugin, blender_workflow::plugin));
}
