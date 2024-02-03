pub(crate) mod character_controller;

pub(crate) mod navigation;
pub(crate) mod physics;

use crate::movement::character_controller::character_controller_plugin;
use crate::movement::navigation::navigation_plugin;
use crate::movement::physics::physics_plugin;
use bevy::prelude::*;
use seldom_fn_plugin::FnPluginExt;

/// This plugin handles all physical movement that is not exclusive to the player.
/// It is further split into the following sub-plugins:
/// - [`physics_plugin`]: Instantiates the rapier integration
/// - [`character_controller_plugin`]: Handles kinematic character controller movement. A "character" in
/// this sense is anything that behaves in a not-quite completely physical way, like a player, an npc, an elevator, a moving platform, etc.
/// Contrast this with pure rigidbodies like a ball, a crate, etc.
/// - [`navigation_plugin`]: Handles npc pathfinding via bevy_pathmesh integration.
pub(crate) fn movement_plugin(app: &mut App) {
    app.fn_plugin(physics_plugin)
        .fn_plugin(character_controller_plugin)
        .fn_plugin(navigation_plugin);
}
