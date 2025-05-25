//! Player pickup UI interactions.
//! In particular, change the crosshair when the player is looking at a prop and hide it when the player is holding a prop.

use std::any::Any as _;

use avian_pickup::{actor::AvianPickupActor, prop::HeldProp};
use avian3d::prelude::*;
use bevy::prelude::*;
#[cfg(feature = "hot_patch")]
use bevy_simple_subsecond_system::hot;

use crate::{
    PostPhysicsAppSystems,
    gameplay::{crosshair::CrosshairState, player::camera::PlayerCamera},
    screens::Screen,
    third_party::avian3d::CollisionLayer,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        check_for_pickup_opportunity
            .run_if(in_state(Screen::Gameplay))
            .in_set(PostPhysicsAppSystems::ChangeUi),
    );
    app.add_observer(hide_crosshair_when_picking_up);
    app.add_observer(show_crosshair_when_not_picking_up);
}

#[cfg_attr(feature = "hot_patch", hot)]
fn check_for_pickup_opportunity(
    player: Single<(&GlobalTransform, &AvianPickupActor), With<PlayerCamera>>,
    spatial_query: SpatialQuery,
    mut crosshair: Single<&mut CrosshairState>,
) {
    let (player, pickup_actor) = player.into_inner();
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

#[cfg_attr(feature = "hot_patch", hot)]
fn hide_crosshair_when_picking_up(
    _trigger: Trigger<OnAdd, HeldProp>,
    mut crosshair: Single<&mut CrosshairState>,
) {
    crosshair
        .wants_invisible
        .insert(hide_crosshair_when_picking_up.type_id());
}

#[cfg_attr(feature = "hot_patch", hot)]
fn show_crosshair_when_not_picking_up(
    _trigger: Trigger<OnRemove, HeldProp>,
    mut crosshair: Single<&mut CrosshairState>,
) {
    crosshair
        .wants_invisible
        .remove(&hide_crosshair_when_picking_up.type_id());
}
