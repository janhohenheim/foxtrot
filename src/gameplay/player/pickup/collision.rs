//! Player pickup collision handling.
//!
//! Disable collision with actors when the player is holding a prop.
//! Re-enable collision when the player is no longer holding a prop.

use bevy::prelude::*;
#[cfg(feature = "hot_patch")]
use bevy_simple_subsecond_system::hot;
use std::iter;

use avian_pickup::prop::HeldProp;
use avian3d::prelude::*;

use crate::third_party::avian3d::CollisionLayer;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(disable_collision_with_held_prop);
    app.add_observer(enable_collision_with_no_longer_held_prop);
}

#[cfg_attr(feature = "hot_patch", hot)]
fn disable_collision_with_held_prop(
    trigger: Trigger<OnAdd, HeldProp>,
    q_children: Query<&Children>,
    mut q_collision_layers: Query<&mut CollisionLayers>,
) {
    let rigid_body = trigger.target();
    for child in iter::once(rigid_body).chain(q_children.iter_descendants(rigid_body)) {
        let Ok(mut collision_layers) = q_collision_layers.get_mut(child) else {
            continue;
        };
        collision_layers.filters.remove(CollisionLayer::Character);
    }
}

#[cfg_attr(feature = "hot_patch", hot)]
fn enable_collision_with_no_longer_held_prop(
    trigger: Trigger<OnRemove, HeldProp>,
    q_children: Query<&Children>,
    mut q_collision_layers: Query<&mut CollisionLayers>,
) {
    let rigid_body = trigger.target();
    for child in iter::once(rigid_body).chain(q_children.iter_descendants(rigid_body)) {
        let Ok(mut collision_layers) = q_collision_layers.get_mut(child) else {
            continue;
        };
        collision_layers.filters.add(CollisionLayer::Character);
    }
}
