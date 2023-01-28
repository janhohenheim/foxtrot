pub mod actions;
pub mod camera;
pub mod player_embodiment;

use crate::player_control::actions::ActionsPlugin;
use crate::player_control::camera::CameraPlugin;
use crate::player_control::player_embodiment::PlayerEmbodimentPlugin;
use bevy::prelude::*;

pub struct PlayerControlPlugin;

impl Plugin for PlayerControlPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ActionsPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(PlayerEmbodimentPlugin);
    }
}
