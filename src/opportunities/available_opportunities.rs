use crate::{
    collision_layer::CollisionLayer,
    player::{camera::PlayerCamera, Player},
};
use avian3d::prelude::*;
use bevy::{prelude::*, utils::HashSet};

use super::OpportunitySystem;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        update_available_opportunities.in_set(OpportunitySystem::UpdateAvailableOpportunities),
    );
    app.register_type::<(OpportunitySensor, AvailableOpportunities)>();
}

/// The general idea is as follows:
/// This sensor is on a collider that is bigger than the object that can be interacted with.
/// When the player stands inside this sensor, we check if a raycast from the camera's forward
/// direction hits the underlying interactable object.
/// Said interactable object is assumed to be the parent of the sensor.
/// For example, a door would have a physics collider, probably a RigidBody::Static.
/// It also has a sensor as a child, with a bigger collider. When the player stands in the
/// sensor, we check if the player has a line of sight to the physical door.
/// If so, we have an interaction opportunity.
#[derive(Debug, Component, PartialEq, Eq, Clone, Reflect)]
#[reflect(Component, PartialEq)]
pub struct OpportunitySensor {
    /// The prompt to display when the opportunity is available.
    pub prompt: String,
    /// The opportunity to activate when the player chooses to interact after the prompt is shown.
    pub opportunity: Opportunity,
}

/// An interaction opportunity stored in an [`OpportunitySensor`]
#[derive(Debug, Clone, Component, PartialEq, Eq, Reflect)]
#[reflect(Component, PartialEq)]
pub enum Opportunity {
    /// A dialog opportunity with a Yarn Spinner dialogue node.
    Dialog(String),
}

/// A set of available opportunities. These are interaction opportunities that have already been
/// validated, i.e. the can interact with them *now* if they choose to.
/// The entities point to the respective [`OpportunitySensor`] holders.
#[derive(Debug, Component, PartialEq, Eq, Clone, Deref, DerefMut, Default, Reflect)]
#[reflect(Component, Default, PartialEq)]
pub struct AvailableOpportunities(HashSet<Entity>);

impl AvailableOpportunities {
    pub fn pick_one(&self) -> Option<Entity> {
        // We could use a variety of strategies to choose the best opportunity,
        // such as prefer talking over interacting with objects.
        // Let's just use the first available opportunity for now.
        // Note that since `HashSet`s have no intrinsic ordering,
        // the chosen opportunity is arbitrary, but consistent until the set changes.
        self.iter().next().copied()
    }
}

fn update_available_opportunities(
    q_dialog_sensor: Query<
        (Entity, &Parent, &CollidingEntities),
        (With<OpportunitySensor>, Changed<CollidingEntities>),
    >,
    mut q_player: Query<(Entity, &mut AvailableOpportunities), With<Player>>,
    q_camera: Query<&Transform, With<PlayerCamera>>,
    spatial_query: SpatialQuery,
) {
    let Ok((player_entity, mut opportunities)) = q_player.get_single_mut() else {
        return;
    };
    let Ok(camera_transform) = q_camera.get_single() else {
        return;
    };
    for (sensor, parent, colliding_entities) in &q_dialog_sensor {
        if !colliding_entities.contains(&player_entity) {
            if opportunities.contains(&sensor) {
                // This used to be an opportunity, but the player has left the sensor.
                opportunities.remove(&sensor);
            }
            continue;
        };
        let underlying_entity = parent.get();

        let origin = camera_transform.translation;
        let direction = camera_transform.forward();
        // Not relevant because we are already inside the sensor,
        // i.e. close enough to interact with the object if nothing is in the way.
        let max_distance = f32::INFINITY;
        // Little bit more efficient than `false`, as we don't care about the actual hit result,
        // only if we hit anything at all.
        let solid = true;
        let query_filter = SpatialQueryFilter::from_mask([
            CollisionLayer::Character,
            CollisionLayer::Prop,
            CollisionLayer::Terrain,
        ])
        .with_excluded_entities([player_entity]);

        let hit = spatial_query.cast_ray(origin, direction, max_distance, solid, &query_filter);

        let has_line_of_sight = hit.is_some_and(|hit| hit.entity == underlying_entity);
        if !has_line_of_sight {
            if opportunities.contains(&sensor) {
                // This used to be an opportunity, but the player does not have a line of sight to the underlying object anymore.
                opportunities.remove(&sensor);
            }
            continue;
        }
        if !opportunities.contains(&sensor) {
            // This is a new opportunity.
            opportunities.insert(sensor);
        }
    }
}
