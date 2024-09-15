use crate::{
    collision_layer::CollisionLayer,
    dialog::conditions::dialog_running,
    player::{camera::PlayerCamera, Player},
};
use avian3d::prelude::*;
use bevy::prelude::*;

use super::OpportunitySystem;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        update_available_opportunities
            .run_if(not(dialog_running))
            .in_set(OpportunitySystem::UpdateAvailableOpportunities),
    );
    app.register_type::<PlayerInteractable>();
}

#[derive(Debug, Component, PartialEq, Eq, Clone, Default, Deref, DerefMut, Reflect)]
#[reflect(Component, PartialEq, Default)]
pub struct ActiveInteractable(pub Option<Entity>);

/// The general idea is as follows:
/// This component sits on a collider for an interactable object, e.g. a door or a character.
/// Every update, we send a raycast from the camera's forward direction to see if it hits a
/// [`PotentialOpportunity`] collider.
/// If so, we have an interaction opportunity.
#[derive(Debug, Component, PartialEq, Clone, Reflect)]
#[reflect(Component, PartialEq)]
pub struct PlayerInteractable {
    /// The prompt to display when the opportunity is available.
    pub prompt: String,
    /// The opportunity to activate when the player chooses to interact after the prompt is shown.
    pub interaction: PlayerInteraction,
    /// The maximum distance from the camera at which the opportunity can be interacted with.
    pub max_distance: f32,
}

#[derive(Debug, Clone, Component, PartialEq, Eq, Reflect)]
#[reflect(Component, PartialEq)]
pub enum PlayerInteraction {
    /// A dialog opportunity with a Yarn Spinner dialogue node.
    Dialog(String),
}

fn update_available_opportunities(
    q_interactable: Query<&PlayerInteractable>,
    mut q_player: Query<(Entity, &mut ActiveInteractable), With<Player>>,
    q_camera: Query<&Transform, With<PlayerCamera>>,
    spatial_query: SpatialQuery,
) {
    let Ok((player_entity, mut active_interactable)) = q_player.get_single_mut() else {
        return;
    };
    let Ok(camera_transform) = q_camera.get_single() else {
        return;
    };

    let origin = camera_transform.translation;
    let direction = camera_transform.forward();
    // Not relevant because we the maximum distance is determined by the object hit by the raycast.
    let max_distance = f32::INFINITY;
    // Little bit more efficient than `false`, as we don't care about the actual hit result,
    // only if we hit anything at all.
    let solid = true;
    // Layers that either contain interactable objects or those able to block line of sight with interactable objects.
    let query_filter = SpatialQueryFilter::from_mask([
        CollisionLayer::Character,
        CollisionLayer::Prop,
        CollisionLayer::Terrain,
    ])
    .with_excluded_entities([player_entity]);

    let interactable = spatial_query
        .cast_ray(origin, direction, max_distance, solid, &query_filter)
        .map(|hit| hit.entity)
        .filter(|entity| q_interactable.contains(*entity));
    let new_interactable = ActiveInteractable(interactable);

    if active_interactable.as_ref() != &new_interactable {
        *active_interactable = new_interactable;
    }
}
