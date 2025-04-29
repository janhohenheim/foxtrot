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
    children: Query<&Children>,
    is_mesh: Query<&Mesh3d>,
    mut commands: Commands,
) {
    let container = trigger.target();
    for child in iter::once(container).chain(children.iter_descendants(container)) {
        if is_mesh.get(child).is_ok() {
            commands
                .entity(child)
                .insert((NotShadowCaster, NotShadowReceiver));
        }
    }
}
