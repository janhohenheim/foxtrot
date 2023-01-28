use crate::shader::Materials;
use crate::util::trait_extension::MeshExt;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
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
        if name.to_lowercase().contains("[collider]") {
            let (collider_entity, collider_mesh) =
                Mesh::search_in_children(children, &meshes, &mesh_handles);
            commands.entity(collider_entity).despawn_recursive();

            let rapier_collider =
                Collider::from_bevy_mesh(collider_mesh, &ComputedColliderShape::TriMesh).unwrap();

            commands.entity(entity).insert(rapier_collider);
        }
    }
}

pub fn set_texture_to_repeat(
    mut commands: Commands,
    added_name: Query<(&Name, &Children), Added<Name>>,
    material_handles: Query<&Handle<StandardMaterial>>,
    materials: Res<Materials>,
) {
    for (name, children) in &added_name {
        if name.to_lowercase().contains("[ground]") {
            let child = children
                .iter()
                .find(|entity| material_handles.get(**entity).is_ok())
                .unwrap();

            commands
                .entity(*child)
                .remove::<Handle<StandardMaterial>>()
                .insert(materials.repeated.clone());
        }
    }
}
