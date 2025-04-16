use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};
use bevy_trenchbroom::prelude::*;

use crate::props::{Candle, generic::dynamic_bundle};

pub(super) fn plugin(_app: &mut App) {}

pub(crate) fn setup_candle(mut world: DeferredWorld, entity: Entity, _id: ComponentId) {
    if world.is_scene_world() {
        return;
    }
    let bundle = dynamic_bundle::<Candle>(&world);
    world.commands().entity(entity).insert(bundle);
}
