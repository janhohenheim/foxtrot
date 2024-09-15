use crate::system_set::VariableGameSystem;
use bevy::prelude::*;

pub mod components;
mod interact;
mod prompt;
mod update_available;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        components::plugin,
        prompt::plugin,
        interact::plugin,
        update_available::plugin,
    ));
    app.configure_sets(
        Update,
        (
            OpportunitySystem::UpdateAvailableOpportunities,
            OpportunitySystem::ShowPrompt,
            OpportunitySystem::Interact,
        )
            .chain()
            .in_set(VariableGameSystem::Opportunities),
    );
}

/// Systems related to opportunities.
///
/// Opportunities are actions that the player can perform, such as interacting with a door.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
enum OpportunitySystem {
    /// Updates the list of available opportunities.
    UpdateAvailableOpportunities,
    /// Shows or hides the prompt for the best available opportunity.
    ShowPrompt,
    /// Handles the player interacting with the best available opportunity.
    Interact,
}
