//! Player pickup collision handling.
//!
//! Disable collision with actors when the player is holding a prop.
//! Re-enable collision when the player is no longer holding a prop.

use bevy::prelude::*;

use std::iter;

use avian_pickup::prop::HeldProp;
use avian3d::prelude::*;

use crate::third_party::avian3d::CollisionLayer;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(disable_collision_with_held_prop);
    app.add_observer(enable_collision_with_no_longer_held_prop);
}

fn disable_collision_with_held_prop(
    add: On<Add, HeldProp>,
    mut commands: Commands,
    q_children: Query<&Children>,
    q_collision_layers: Query<&CollisionLayers>,
) {
    let rigid_body = add.entity;
    for child in iter::once(rigid_body).chain(q_children.iter_descendants(rigid_body)) {
        let Ok(mut collision_layers) = q_collision_layers.get(child).copied() else {
            continue;
        };
        collision_layers.filters.remove(CollisionLayer::Character);
        commands.entity(child).insert(collision_layers);
    }
}

fn enable_collision_with_no_longer_held_prop(
    remove: On<Remove, HeldProp>,
    mut commands: Commands,
    q_children: Query<&Children>,
    q_collision_layers: Query<&CollisionLayers>,
) {
    let rigid_body = remove.entity;
    for child in iter::once(rigid_body).chain(q_children.iter_descendants(rigid_body)) {
        let Ok(mut collision_layers) = q_collision_layers.get(child).copied() else {
            continue;
        };
        collision_layers.filters.add(CollisionLayer::Character);
        commands.entity(child).insert(collision_layers);
    }
}
