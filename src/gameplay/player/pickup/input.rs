//! Forward the player's input to the pickup plugin.

use bevy::prelude::*;
#[cfg(feature = "hot_patch")]
use bevy_simple_subsecond_system::hot;

use avian_pickup::prelude::*;
use bevy_enhanced_input::prelude::*;

use crate::gameplay::player::default_input::{DropProp, PickupProp};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(pull_prop);
    app.add_observer(throw_prop);
    app.add_observer(drop_prop);
}

#[cfg_attr(feature = "hot_patch", hot)]
fn pull_prop(
    _trigger: Trigger<Fired<PickupProp>>,
    actor: Single<Entity, With<AvianPickupActor>>,
    mut avian_pickup_input_writer: EventWriter<AvianPickupInput>,
) {
    avian_pickup_input_writer.write(AvianPickupInput {
        action: AvianPickupAction::Pull,
        actor: *actor,
    });
}

#[cfg_attr(feature = "hot_patch", hot)]
fn throw_prop(
    _trigger: Trigger<Started<PickupProp>>,
    actor: Single<Entity, With<AvianPickupActor>>,
    mut avian_pickup_input_writer: EventWriter<AvianPickupInput>,
) {
    avian_pickup_input_writer.write(AvianPickupInput {
        action: AvianPickupAction::Throw,
        actor: *actor,
    });
}

#[cfg_attr(feature = "hot_patch", hot)]
fn drop_prop(
    _trigger: Trigger<Started<DropProp>>,
    actor: Single<Entity, With<AvianPickupActor>>,
    mut avian_pickup_input_writer: EventWriter<AvianPickupInput>,
) {
    avian_pickup_input_writer.write(AvianPickupInput {
        action: AvianPickupAction::Drop,
        actor: *actor,
    });
}
