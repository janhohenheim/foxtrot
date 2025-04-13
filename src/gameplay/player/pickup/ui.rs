use std::any::Any as _;

use avian_pickup::{actor::AvianPickupActor, prop::HeldProp};
use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    gameplay::{cursor::CrosshairState, player::camera::PlayerCameraParent},
    third_party::avian3d::CollisionLayer,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (check_for_pickup_opportunity,));
    app.add_observer(hide_crosshair_when_picking_up);
    app.add_observer(show_crosshair_when_not_picking_up);
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
    let system_id = check_for_pickup_opportunity.type_id();
    if hit.is_some() {
        crosshair.wants_square.insert(system_id);
    } else {
        crosshair.wants_square.remove(&system_id);
    }
}

fn hide_crosshair_when_picking_up(
    _trigger: Trigger<OnAdd, HeldProp>,
    crosshair: Option<Single<&mut CrosshairState>>,
) {
    let Some(mut crosshair) = crosshair else {
        return;
    };
    crosshair
        .wants_invisible
        .insert(hide_crosshair_when_picking_up.type_id());
}

fn show_crosshair_when_not_picking_up(
    _trigger: Trigger<OnRemove, HeldProp>,
    crosshair: Option<Single<&mut CrosshairState>>,
) {
    let Some(mut crosshair) = crosshair else {
        return;
    };
    crosshair
        .wants_invisible
        .remove(&hide_crosshair_when_picking_up.type_id());
}
