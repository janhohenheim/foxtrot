pub mod condition;
pub mod dialog;
pub mod interactions_ui;

use crate::world_interaction::condition::ConditionPlugin;
use crate::world_interaction::dialog::DialogPlugin;
use crate::world_interaction::interactions_ui::InteractionsUiPlugin;
use bevy::prelude::*;

/// Handles player to world interactions. Split in to the following sub-plugins:
/// - [`ConditionPlugin`] handles trackers of player actions such as chosen dialog options
/// - [`DialogPlugin`] handles dialog trees
/// - [`InteractionsUiPlugin`] handles the UI for interacting with an object in front of the player.
pub struct WorldInteractionPlugin;

impl Plugin for WorldInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ConditionPlugin)
            .add_plugin(DialogPlugin)
            .add_plugin(InteractionsUiPlugin);
    }
}
