use avian3d::prelude::ColliderParent;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{character::action::CharacterAction, dialog::StartDialog, player::Player};

use super::{
    available_opportunities::{AvailableOpportunities, Opportunity, OpportunitySensor},
    OpportunitySystem,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, usher_interact.in_set(OpportunitySystem::Interact));
}

/// Triggers dialog opportunity events when the player has an interaction opportunity and presses
/// the interact button. The target entity is the entity that has the [`RigidBody`] component.
fn usher_interact(
    mut q_player: Query<(&ActionState<CharacterAction>, &mut AvailableOpportunities), With<Player>>,
    q_opportunity_sensor: Query<(&OpportunitySensor, &ColliderParent)>,
    mut commands: Commands,
) {
    for (action_state, mut opportunities) in &mut q_player {
        if !action_state.just_pressed(&CharacterAction::Interact) {
            continue;
        }
        let Some(opportunity) = opportunities.pick_one() else {
            continue;
        };
        let Ok((sensor, rigid_body)) = q_opportunity_sensor.get(opportunity) else {
            // Looks like the opportunity despawned.
            opportunities.remove(&opportunity);
            continue;
        };
        match &sensor.opportunity {
            Opportunity::Dialog(node) => {
                commands.trigger_targets(StartDialog(node.clone()), rigid_body.get());
            }
        }
    }
}
