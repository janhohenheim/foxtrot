//! Utility functions for adding special effects to props.

use bevy::{pbr::NotShadowCaster, prelude::*, scene::SceneInstanceReady};
#[cfg(feature = "hot_patch")]
use bevy_simple_subsecond_system::hot;
use std::iter;

pub(super) fn plugin(_app: &mut App) {}

#[cfg_attr(feature = "hot_patch", hot)]
pub(crate) fn disable_shadow_casting_on_instance_ready(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
) {
    commands
        .entity(trigger.target())
        .queue(disable_shadow_casting);
}

pub(crate) fn disable_shadow_casting(entity_world: EntityWorldMut) {
    let entity = entity_world.id();
    entity_world
        .into_world_mut()
        .run_system_cached_with(disable_shadow_casting_system, entity)
        .unwrap();
}

fn disable_shadow_casting_system(
    In(entity): In<Entity>,
    children: Query<&Children>,
    is_mesh: Query<&Mesh3d>,
    mut commands: Commands,
) {
    for child in iter::once(entity).chain(children.iter_descendants(entity)) {
        if is_mesh.get(child).is_ok() {
            commands.entity(child).insert(NotShadowCaster);
        }
    }
}
