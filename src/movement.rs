pub mod audio;
pub mod general_movement;
pub mod navigation;
pub mod physics;

use crate::movement::audio::InternalAudioPlugin;
use crate::movement::general_movement::GeneralMovementPlugin;
use crate::movement::navigation::NavigationPlugin;
use crate::movement::physics::PhysicsPlugin;
use bevy::prelude::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PhysicsPlugin)
            .add_plugin(GeneralMovementPlugin)
            .add_plugin(NavigationPlugin)
            .add_plugin(InternalAudioPlugin);
    }
}
