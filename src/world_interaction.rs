pub mod condition;
pub mod dialog;
pub mod interactions_ui;

use crate::world_interaction::condition::ConditionPlugin;
use crate::world_interaction::dialog::DialogPlugin;
use crate::world_interaction::interactions_ui::InteractionsUiPlugin;
use bevy::prelude::*;
use seldom_fn_plugin::FnPluginExt;

/// Handles player to world interactions. Split in to the following sub-plugins:
/// - [`ConditionPlugin`] handles trackers of player actions such as chosen dialog options
/// - [`DialogPlugin`] handles dialog trees
/// - [`InteractionsUiPlugin`] handles the UI for interacting with an object in front of the player.
pub fn WorldInteractionPlugin(app: &mut App) {
    app.fn_plugin(ConditionPlugin)
        .fn_plugin(DialogPlugin)
        .fn_plugin(InteractionsUiPlugin);
}
