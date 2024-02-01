pub(crate) mod dialog;
pub(crate) mod interactions_ui;

use crate::world_interaction::dialog::dialog_plugin;
use crate::world_interaction::interactions_ui::interactions_ui_plugin;
use bevy::prelude::*;
use seldom_fn_plugin::FnPluginExt;

/// Handles player to world interactions. Split in to the following sub-plugins:
/// - [`dialog_plugin`] handles dialog trees
/// - [`interactions_ui_plugin`] handles the UI for interacting with an object in front of the player.
pub(crate) fn world_interaction_plugin(app: &mut App) {
    app.fn_plugin(dialog_plugin)
        .fn_plugin(interactions_ui_plugin);
}
