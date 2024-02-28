use crate::{level_instantiation::spawning::objects::CollisionLayer, GameState};
use anyhow::Context;
use bevy::{prelude::*, transform::TransformSystem::TransformPropagate};
use bevy_mod_sysfail::prelude::*;
use bevy_xpbd_3d::prelude::*;
use oxidized_navigation::NavMeshAffector;
use serde::{Deserialize, Serialize};

/// Sets up the [`RapierPhysicsPlugin`] and [`RapierConfiguration`].
pub(crate) fn physics_plugin(app: &mut App) {
    app.register_type::<ColliderMarker>()
        .add_plugins(PhysicsPlugins::default())
        // Using the default fixed timestep causes issues on faster (165 Hz) machines.
        .insert_resource(Time::new_with(Physics::variable(1.0 / 60.)))
        .add_systems(
            Update,
            read_colliders
                .run_if(in_state(GameState::Playing))
                .after(TransformPropagate),
        );
}

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub(crate) struct ColliderMarker;

#[sysfail(Log<anyhow::Error, Error>)]
pub(crate) fn read_colliders(
    collider_marker: Query<Entity, With<ColliderMarker>>,
    mut commands: Commands,
    children: Query<&Children>,
    meshes: Res<Assets<Mesh>>,
    mesh_handles: Query<&Handle<Mesh>, Without<RigidBody>>,
    global_transforms: Query<&GlobalTransform>,
) {
    #[cfg(feature = "tracing")]
    let _span = info_span!("read_colliders").entered();
    for entity in collider_marker.iter() {
        let mut all_children_loaded = true;
        for child in children.iter_descendants(entity) {
            if let Ok(mesh_handle) = mesh_handles.get(child) {
                if let Some(mesh) = meshes.get(mesh_handle) {
                    let global_transform = global_transforms
                        .get(child)
                        .context("Failed to get global transform while reading collider")?
                        .compute_transform();
                    let scaled_mesh = mesh.clone().scaled_by(global_transform.scale);
                    let collider = Collider::trimesh_from_mesh(&scaled_mesh)
                        .context("Failed to create collider from mesh")?;
                    commands.entity(child).insert((
                        collider,
                        RigidBody::Static,
                        CollisionLayers::new(
                            [CollisionLayer::Terrain, CollisionLayer::CameraObstacle],
                            [CollisionLayer::Character],
                        ),
                        NavMeshAffector,
                    ));
                } else {
                    all_children_loaded = false;
                }
            }
        }
        if all_children_loaded {
            commands.entity(entity).remove::<ColliderMarker>();
        }
    }
}
