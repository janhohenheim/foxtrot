//! Utility functions for adding special effects to props.

use std::iter;

use bevy::{
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
    scene::SceneInstanceReady,
};

pub(super) fn plugin(_app: &mut App) {}

pub(crate) fn prepare_light_mesh(
    trigger: Trigger<SceneInstanceReady>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    children: Query<&Children>,
    handles: Query<&MeshMaterial3d<StandardMaterial>>,
    is_mesh: Query<&Mesh3d>,
    mut commands: Commands,
) {
    let container = trigger.entity();
    for child in iter::once(container).chain(children.iter_descendants(container)) {
        if is_mesh.get(child).is_ok() {
            commands
                .entity(child)
                .insert((NotShadowCaster, NotShadowReceiver));
        }
        if let Ok(material) = handles.get(child) {
            let Some(material) = materials.get_mut(material.id()) else {
                error!("Failed to mutate a material at runtime. Did you forget to preload it?");
                continue;
            };
            material.unlit = true;
        }
    }
}
