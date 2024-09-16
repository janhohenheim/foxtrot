use crate::{
    collision_layer::CollisionLayer,
    dialog::conditions::dialog_running,
    player::{camera::PlayerCamera, Player},
};
use avian3d::prelude::*;
use bevy::prelude::*;

use super::{
    OpportunitySystem, {AvailablePlayerInteraction, PlayerInteraction, PlayerInteractionParameters},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        update_available_interaction
            .run_if(not(dialog_running))
            .in_set(OpportunitySystem::UpdateAvailableOpportunities),
    );
}

fn update_available_interaction(
    q_interaction: Query<&PlayerInteractionParameters, With<PlayerInteraction>>,
    mut q_player: Query<(Entity, &mut AvailablePlayerInteraction), With<Player>>,
    q_camera: Query<&Transform, With<PlayerCamera>>,
    q_collider_parent: Query<&ColliderParent>,
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

    let mut new_interactable = None;
    if let Some(hit) = spatial_query.cast_ray(origin, direction, max_distance, solid, &query_filter)
    {
        if let Ok(collider_parent) = q_collider_parent.get(hit.entity) {
            let collider_parent = collider_parent.get();
            if let Ok(interaction_params) = q_interaction.get(collider_parent) {
                if hit.time_of_impact <= interaction_params.max_distance {
                    new_interactable = Some(collider_parent);
                }
            }
        }
    }
    let new_interactable = AvailablePlayerInteraction(new_interactable);

    // Be nice to change detection :)
    if active_interactable.as_ref() != &new_interactable {
        *active_interactable = new_interactable;
    }
}
