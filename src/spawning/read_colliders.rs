use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use oxidized_navigation::NavMeshAffector;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct CustomCollider;

#[allow(clippy::type_complexity)]
pub fn read_colliders(
    mut commands: Commands,
    added_name: Query<(Entity, &Name, &Children), (Added<Name>, Without<CustomCollider>)>,
    meshes: Res<Assets<Mesh>>,
    mesh_handles: Query<&Handle<Mesh>>,
) {
    for (entity, name, children) in &added_name {
        if name.to_lowercase().contains("collider") {
            let colliders: Vec<_> = children
                .iter()
                .filter_map(|entity| mesh_handles.get(*entity).ok().map(|mesh| (*entity, mesh)))
                .collect();
            assert_eq!(
                colliders.len(),
                1,
                "Collider must contain exactly one mesh, but found {}",
                colliders.len()
            );
            let (collider_entity, collider_mesh_handle) = colliders.first().unwrap();
            let collider_mesh = meshes.get(collider_mesh_handle).unwrap();
            commands.entity(*collider_entity).despawn_recursive();

            let rapier_collider =
                Collider::from_bevy_mesh(collider_mesh, &ComputedColliderShape::TriMesh).unwrap();

            commands
                .entity(entity)
                .insert((rapier_collider, NavMeshAffector::default()));
        }
    }
}
