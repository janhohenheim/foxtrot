pub mod condition;
pub mod dialog;
pub mod interactions_ui;

use crate::world_interaction::condition::ConditionPlugin;
use crate::world_interaction::dialog::DialogPlugin;
use crate::world_interaction::interactions_ui::InteractionsUiPlugin;
use bevy::prelude::*;

pub struct WorldInteractionPlugin;

impl Plugin for WorldInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ConditionPlugin)
            .add_plugin(DialogPlugin)
            .add_plugin(InteractionsUiPlugin);
    }
}
