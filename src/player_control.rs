pub mod actions;
pub mod camera;
pub mod player_embodiment;

pub use crate::player_control::actions::ActionsPlugin;
pub use crate::player_control::camera::CameraPlugin;
pub use crate::player_control::player_embodiment::PlayerEmbodimentPlugin;
use bevy::prelude::*;
use seldom_fn_plugin::FnPluginExt;

/// Handles systems exclusive to the player's control. Is split into the following sub-plugins:
/// - [`ActionsPlugin`]: Handles player input such as mouse and keyboard and neatly packs it into an [`actions::Actions`] resource.
/// - [`CameraPlugin`]: Handles camera movement.
/// - [`PlayerEmbodimentPlugin`]: Tells the components from [`super::MovementPlugin`] about the desired player [`actions::Actions`].
/// Also handles other systems that change how the player is physically represented in the world.
pub fn PlayerControlPlugin(app: &mut App) {
    app.fn_plugin(ActionsPlugin)
        .fn_plugin(CameraPlugin)
        .fn_plugin(PlayerEmbodimentPlugin);
}
