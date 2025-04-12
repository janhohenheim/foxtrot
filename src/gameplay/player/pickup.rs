use std::iter;

use avian_pickup::{prelude::*, prop::HeldProp};
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_enhanced_input::prelude::*;

use crate::third_party::avian3d::CollisionLayer;

use super::default_input::{PickupProp, ThrowProp};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(pull_prop);
    app.add_observer(throw_prop);
    app.add_observer(drop_prop);
    app.add_observer(disable_collision_with_held_prop);
    app.add_observer(enable_collision_with_no_longer_held_prop);
}

fn pull_prop(
    _trigger: Trigger<Fired<PickupProp>>,
    actors: Option<Single<Entity, With<AvianPickupActor>>>,
    mut avian_pickup_input_writer: EventWriter<AvianPickupInput>,
) {
    let Some(actor) = actors else {
        return;
    };
    avian_pickup_input_writer.send(AvianPickupInput {
        action: AvianPickupAction::Pull,
        actor: *actor,
    });
}

fn throw_prop(
    _trigger: Trigger<Started<ThrowProp>>,
    actors: Option<Single<Entity, With<AvianPickupActor>>>,
    mut avian_pickup_input_writer: EventWriter<AvianPickupInput>,
) {
    let Some(actor) = actors else {
        return;
    };
    avian_pickup_input_writer.send(AvianPickupInput {
        action: AvianPickupAction::Throw,
        actor: *actor,
    });
}

fn drop_prop(
    _trigger: Trigger<Started<PickupProp>>,
    actors: Option<Single<Entity, With<AvianPickupActor>>>,
    mut avian_pickup_input_writer: EventWriter<AvianPickupInput>,
) {
    let Some(actor) = actors else {
        return;
    };
    avian_pickup_input_writer.send(AvianPickupInput {
        action: AvianPickupAction::Drop,
        actor: *actor,
    });
}

fn disable_collision_with_held_prop(
    trigger: Trigger<OnAdd, HeldProp>,
    q_children: Query<&Children>,
    mut q_collision_layers: Query<&mut CollisionLayers>,
) {
    let rigid_body = trigger.entity();
    for child in iter::once(rigid_body).chain(q_children.iter_descendants(rigid_body)) {
        let Ok(mut collision_layers) = q_collision_layers.get_mut(child) else {
            continue;
        };
        collision_layers.filters.remove(CollisionLayer::Player);
    }
}

fn enable_collision_with_no_longer_held_prop(
    trigger: Trigger<OnRemove, HeldProp>,
    q_children: Query<&Children>,
    mut q_collision_layers: Query<&mut CollisionLayers>,
) {
    let rigid_body = trigger.entity();
    for child in iter::once(rigid_body).chain(q_children.iter_descendants(rigid_body)) {
        let Ok(mut collision_layers) = q_collision_layers.get_mut(child) else {
            continue;
        };
        collision_layers.filters.add(CollisionLayer::Player);
    }
}
