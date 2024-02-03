use crate::level_instantiation::spawning::objects::CollisionLayer;
use crate::GameState;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy_mod_sysfail::*;
use bevy_xpbd_3d::prelude::*;
use oxidized_navigation::NavMeshAffector;
use serde::{Deserialize, Serialize};

/// Sets up the [`RapierPhysicsPlugin`] and [`RapierConfiguration`].
pub(crate) fn physics_plugin(app: &mut App) {
    app.register_type::<ColliderMarker>()
        .add_plugins(PhysicsPlugins::default())
        // Using the default fixed timestep causes issues on faster (165 Hz) machines.
        .insert_resource(Time::new_with(Physics::variable(1.0 / 60.)))
        .add_systems(Update, read_colliders.run_if(in_state(GameState::Playing)));
}

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub(crate) struct ColliderMarker;

#[sysfail(log(level = "error"))]
pub(crate) fn read_colliders(
    collider_marker: Query<Entity, Added<ColliderMarker>>,
    mut commands: Commands,
    children: Query<&Children>,
    meshes: Res<Assets<Mesh>>,
    mesh_handles: Query<&Handle<Mesh>>,
) -> Result<()> {
    #[cfg(feature = "tracing")]
    let _span = info_span!("read_colliders").entered();
    for entity in collider_marker.iter() {
        let mesh = find_mesh(entity, &children, &meshes, &mesh_handles)
            .context("Failed to find mesh for collider")?;
        let collider =
            Collider::trimesh_from_mesh(mesh).context("Failed to create collider from mesh")?;

        commands.entity(entity).insert((
            collider,
            RigidBody::Static,
            CollisionLayers::new(
                [CollisionLayer::Terrain, CollisionLayer::CameraObstacle],
                [CollisionLayer::Character],
            ),
            NavMeshAffector,
        ));
    }
    Ok(())
}

fn find_mesh<'a>(
    parent: Entity,
    children_query: &'a Query<&Children>,
    meshes: &'a Assets<Mesh>,
    mesh_handles: &'a Query<&Handle<Mesh>>,
) -> Option<&'a Mesh> {
    if let Ok(children) = children_query.get(parent) {
        for child in children.iter() {
            if let Ok(mesh_handle) = mesh_handles.get(*child) {
                if let Some(mesh) = meshes.get(mesh_handle) {
                    return Some(mesh);
                }
            }
        }
    }
    None
}
