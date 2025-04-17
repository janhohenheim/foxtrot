use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};
use bevy_trenchbroom::util::IsSceneWorld as _;

use crate::props::{BurningLogs, generic::static_bundle};

pub(super) fn plugin(_app: &mut App) {}

pub(crate) fn setup_burning_logs(mut world: DeferredWorld, entity: Entity, _id: ComponentId) {
    if world.is_scene_world() {
        return;
    }
    let bundle = static_bundle::<BurningLogs>(&world);
    world.commands().entity(entity).insert(bundle);
}
