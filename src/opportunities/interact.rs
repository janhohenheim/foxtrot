use avian3d::prelude::ColliderParent;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{character::action::CharacterAction, dialog::StartDialog, player::Player};

use super::{
    available_opportunities::{ActiveInteractable, PlayerInteractable, PlayerInteraction},
    OpportunitySystem,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, usher_interact.in_set(OpportunitySystem::Interact));
}

/// Triggers dialog opportunity events when the player has an interaction opportunity and presses
/// the interact button. The target entity is the entity that has the [`RigidBody`] component.
fn usher_interact(
    mut q_player: Query<(&ActionState<CharacterAction>, &mut ActiveInteractable), With<Player>>,
    q_interactable: Query<(&PlayerInteractable, &ColliderParent)>,
    mut commands: Commands,
) {
    for (action_state, mut active_interactable) in &mut q_player {
        if !action_state.just_pressed(&CharacterAction::Interact) {
            continue;
        }
        let Some((interactable, rigid_body)) = active_interactable
            // Clear the current interactable so that it won't show up if we end up in a dialog
            .take()
            .and_then(|e| q_interactable.get(e).ok())
        else {
            continue;
        };
        match &interactable.interaction {
            PlayerInteraction::Dialog(node) => {
                commands.trigger_targets(StartDialog(node.clone()), rigid_body.get());
            }
        }
    }
}
