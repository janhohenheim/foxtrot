use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{character::action::CharacterAction, dialog::StartDialog, player::Player};

use super::{
    OpportunitySystem, {AvailablePlayerInteraction, PlayerInteraction},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, usher_interact.in_set(OpportunitySystem::Interact));
}

/// Triggers dialog opportunity events when the player has an interaction opportunity and presses
/// the interact button. The target entity is the entity that has the [`RigidBody`] component.
fn usher_interact(
    mut q_player: Query<
        (
            &ActionState<CharacterAction>,
            &mut AvailablePlayerInteraction,
        ),
        With<Player>,
    >,
    q_interaction: Query<(Entity, &PlayerInteraction)>,
    mut commands: Commands,
) {
    for (action_state, mut active_interactable) in &mut q_player {
        if active_interactable.is_none() || !action_state.just_pressed(&CharacterAction::Interact) {
            continue;
        }
        let Some((entity, interaction)) = active_interactable
            // Clear the current interactable so that it won't show up if we end up in a dialog
            .take()
            .and_then(|e| q_interaction.get(e).ok())
        else {
            continue;
        };
        match interaction {
            PlayerInteraction::Dialog(node) => {
                commands.trigger_targets(StartDialog(node.clone()), entity);
            }
        }
    }
}
