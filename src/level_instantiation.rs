use crate::level_instantiation::{grass::grass_plugin, map::map_plugin, spawning::spawning_plugin};
use bevy::prelude::*;
use seldom_fn_plugin::FnPluginExt;

pub(crate) mod grass;
pub(crate) mod map;
pub(crate) mod spawning;

/// Handles creation of levels and objects. Split into the following sub-plugins:
/// - [`map_plugin`] handles loading of level files and orchestrates the spawning of the objects therein.
/// - [`spawning_plugin`] handles the spawning of objects in general.
/// - [`grass_plugin`] handles the spawning of grass on top of marked meshes.
pub(crate) fn level_instantiation_plugin(app: &mut App) {
    app.fn_plugin(map_plugin)
        .fn_plugin(spawning_plugin)
        .fn_plugin(grass_plugin);
}
