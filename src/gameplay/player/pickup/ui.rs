use avian_pickup::actor::AvianPickupActor;
use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    gameplay::{cursor::CrosshairState, player::camera::PlayerCameraParent},
    third_party::avian3d::CollisionLayer,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, check_for_pickup_opportunity);
}

fn check_for_pickup_opportunity(
    player: Option<Single<(&GlobalTransform, &AvianPickupActor), With<PlayerCameraParent>>>,
    spatial_query: SpatialQuery,
    crosshair: Option<Single<&mut CrosshairState>>,
) {
    let Some((player, pickup_actor)) = player.map(|p| p.into_inner()) else {
        return;
    };
    let Some(mut crosshair) = crosshair else {
        return;
    };
    let camera_transform = player.compute_transform();
    let hit = spatial_query.cast_ray(
        camera_transform.translation,
        camera_transform.forward(),
        pickup_actor.interaction_distance,
        true,
        &SpatialQueryFilter::from_mask(CollisionLayer::Prop),
    );
    if hit.is_some() {
        **crosshair = CrosshairState::Square;
    } else {
        **crosshair = CrosshairState::Dot;
    }
}
