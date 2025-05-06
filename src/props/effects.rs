//! Utility functions for adding special effects to props.

use std::iter;

use bevy::{
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
    scene::SceneInstanceReady,
};

pub(super) fn plugin(_app: &mut App) {}

pub(crate) fn prepare_light_meshes_on_instace_ready(
    trigger: Trigger<SceneInstanceReady>,
    mut commands: Commands,
) {
    commands.entity(trigger.target()).queue(prepare_meshes);
}

pub(crate) fn prepare_meshes(entity_world: EntityWorldMut) {
    let entity = entity_world.id();
    entity_world
        .into_world_mut()
        .run_system_cached_with(prepare_light_meshes_system, entity)
        .unwrap();
}

fn prepare_light_meshes_system(
    In(entity): In<Entity>,
    children: Query<&Children>,
    is_mesh: Query<&Mesh3d>,
    mut commands: Commands,
) {
    for child in iter::once(entity).chain(children.iter_descendants(entity)) {
        if is_mesh.get(child).is_ok() {
            commands
                .entity(child)
                .insert((NotShadowCaster, NotShadowReceiver));
        }
    }
}
