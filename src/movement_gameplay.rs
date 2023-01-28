pub mod actions;
pub mod audio;
pub mod camera;
pub mod general_movement;
pub mod navigation;
pub mod physics;
pub mod player;

use bevy::prelude::*;
use crate::movement_gameplay::camera::CameraPlugin;
use crate::movement_gameplay::general_movement::MovementPlugin;
use crate::movement_gameplay::navigation::NavigationPlugin;
use crate::movement_gameplay::player::PlayerPlugin;
use crate::movement_gameplay::actions::ActionsPlugin;
use crate::movement_gameplay::audio::InternalAudioPlugin;
use crate::movement_gameplay::physics::PhysicsPlugin;

pub struct MovementGameplayPlugin;

impl Plugin for MovementGameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ActionsPlugin)
            .add_plugin(PhysicsPlugin)
            .add_plugin(MovementPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(NavigationPlugin)
            .add_plugin(InternalAudioPlugin);
    }
}
