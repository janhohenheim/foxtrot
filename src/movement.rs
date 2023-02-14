pub mod general_movement;
pub mod navigation;
pub mod physics;

use crate::movement::general_movement::GeneralMovementPlugin;
use crate::movement::navigation::NavigationPlugin;
use crate::movement::physics::PhysicsPlugin;
use bevy::prelude::*;

/// This plugin handles all physical movement that is not exclusive to the player.
/// It is further split into the following sub-plugins:
/// - [`PhysicsPlugin`]: Instantiates the rapier integration
/// - [`GeneralMovementPlugin`]: Handles kinematic character controller movement. A "character" in
/// this sense is anything that behaves in a not-quite completely physical way, like a player, an npc, an elevator, a moving platform, etc.
/// Contrast this with pure rigidbodies like a ball, a crate, etc.
/// - [`NavigationPlugin`]: Handles npc pathfinding via bevy_pathmesh integration.
pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PhysicsPlugin)
            .add_plugin(GeneralMovementPlugin)
            .add_plugin(NavigationPlugin);
    }
}
