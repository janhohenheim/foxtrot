use bevy::prelude::*;

pub(crate) mod character_controller;

mod navigation;
pub(crate) mod physics;

/// This plugin handles all physical movement that is not exclusive to the player.
/// It is further split into the following sub-plugins:
/// - [`physics::plugin`]: Instantiates the rapier integration
/// - [`character_controller::plugin`]: Handles kinematic character controller movement. A "character" in
/// this sense is anything that behaves in a not-quite completely physical way, like a player, an npc, an elevator, a moving platform, etc.
/// Contrast this with pure rigidbodies like a ball, a crate, etc.
/// - [`navigation::plugin`]: Handles npc pathfinding via bevy_pathmesh integration.
pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        physics::plugin,
        character_controller::plugin,
        navigation::plugin,
    ));
}
