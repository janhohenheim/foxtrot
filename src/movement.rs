pub(crate) mod general_movement;
pub(crate) mod navigation;
pub(crate) mod physics;

use crate::movement::general_movement::general_movement_plugin;
use crate::movement::navigation::navigation_plugin;
use crate::movement::physics::physics_plugin;
use bevy::prelude::*;
use seldom_fn_plugin::FnPluginExt;

/// This plugin handles all physical movement that is not exclusive to the player.
/// It is further split into the following sub-plugins:
/// - [`physics_plugin`]: Instantiates the rapier integration
/// - [`general_movement_plugin`]: Handles kinematic character controller movement. A "character" in
/// this sense is anything that behaves in a not-quite completely physical way, like a player, an npc, an elevator, a moving platform, etc.
/// Contrast this with pure rigidbodies like a ball, a crate, etc.
/// - [`navigation_plugin`]: Handles npc pathfinding via bevy_pathmesh integration.
pub(crate) fn movement_plugin(app: &mut App) {
    app.fn_plugin(physics_plugin)
        .fn_plugin(general_movement_plugin)
        .fn_plugin(navigation_plugin);
}
