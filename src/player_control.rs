use bevy::prelude::*;

pub(crate) mod actions;
pub(crate) mod camera;
pub(crate) mod player_embodiment;

/// Handles systems exclusive to the player's control. Is split into the following sub-plugins:
/// - [`actions::plugin`]: Handles player input such as mouse and keyboard and neatly packs it into a [`leafwing_input_manager::Actionlike`].
/// - [`camera::plugin`]: Handles camera movement.
/// - [`player_embodiment::plugin`]: Tells the components from [`super::movement::plugin`] about the desired [`actions::PlayerAction`]s.
/// Also handles other systems that change how the player is physically represented in the world.
pub(super) fn plugin(app: &mut App) {
    app.add_plugins((actions::plugin, camera::plugin, player_embodiment::plugin));
}
