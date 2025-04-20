//! Utility functions for adding special effects to props.

use bevy::{
    pbr::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
    scene::SceneInstanceReady,
};

pub(super) fn plugin(_app: &mut App) {}

pub(crate) fn insert_not_shadow_caster(
    trigger: Trigger<SceneInstanceReady>,
    is_mesh: Query<&Mesh3d>,
    children: Query<&Children>,
    mut commands: Commands,
) {
    for child in children
        .iter_descendants(trigger.entity())
        .filter(|e| is_mesh.get(*e).is_ok())
    {
        commands
            .entity(child)
            .insert((NotShadowCaster, NotShadowReceiver));
    }
}
