pub(crate) use crate::player_control::{
    actions::actions_plugin, camera::camera_plugin, player_embodiment::player_embodiment_plugin,
};
use bevy::prelude::*;
use seldom_fn_plugin::FnPluginExt;

pub(crate) mod actions;
pub(crate) mod camera;
pub(crate) mod player_embodiment;

/// Handles systems exclusive to the player's control. Is split into the following sub-plugins:
/// - [`actions_plugin`]: Handles player input such as mouse and keyboard and neatly packs it into an [`actions::Actions`] resource.
/// - [`camera_plugin`]: Handles camera movement.
/// - [`player_embodiment_plugin`]: Tells the components from [`super::movement_plugin`] about the desired player [`actions::Actions`].
/// Also handles other systems that change how the player is physically represented in the world.
pub(crate) fn player_control_plugin(app: &mut App) {
    app.fn_plugin(actions_plugin)
        .fn_plugin(camera_plugin)
        .fn_plugin(player_embodiment_plugin);
}
