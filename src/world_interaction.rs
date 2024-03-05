use bevy::prelude::*;

pub(crate) mod dialog;
mod interaction_ui;

/// Handles player to world interactions. Split into the following sub-plugins:
/// - [`dialog::plugin`] handles dialog trees
/// - [`interaction_ui::plugin`] handles the UI for interacting with an object in front of the player.
pub(super) fn plugin(app: &mut App) {
    app.add_plugins((dialog::plugin, interaction_ui::plugin));
}
