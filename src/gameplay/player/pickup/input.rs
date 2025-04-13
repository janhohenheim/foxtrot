use bevy::prelude::*;

use avian_pickup::prelude::*;
use bevy_enhanced_input::prelude::*;

use crate::gameplay::player::default_input::{PickupProp, ThrowProp};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(pull_prop);
    app.add_observer(throw_prop);
    app.add_observer(drop_prop);
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
