use crate::shader::Materials;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::render::mesh::{
    MeshVertexAttribute, MeshVertexAttributeId, PrimitiveTopology, VertexAttributeValues,
};
use bevy_pathmesh::PathMesh;
use bevy_rapier3d::prelude::*;
use itertools::Itertools;
use ordered_float::OrderedFloat;
use polyanya::Vertex;
use serde::{Deserialize, Serialize};
use std::borrow::BorrowMut;
use std::iter;

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
            let (collider_entity, collider_mesh) = get_mesh(children, &meshes, &mesh_handles);
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

pub fn read_navmesh(
    mut commands: Commands,
    added_name: Query<(Entity, &Name, &Children), Added<Name>>,
    parents: Query<&Parent>,
    transforms: Query<&Transform>,
    meshes: Res<Assets<Mesh>>,
    mesh_handles: Query<&Handle<Mesh>>,
    mut path_meshes: ResMut<Assets<PathMesh>>,
) {
    for (parent, name, children) in &added_name {
        if name.to_lowercase().contains("[navmesh]") {
            // Necessary because at this stage the `GlobalTransform` is still at `default()` for some reason
            let global_transform = get_global_transform(parent, &parents, &transforms);
            let (child, mesh) = get_mesh(children, &meshes, &mesh_handles);
            let mesh = transform_mesh(mesh, global_transform);
            let path_mesh = PathMesh::from_bevy_mesh_and_then(&mesh, |mesh| {
                mesh.set_delta(1.);
            });
            commands.entity(parent).insert(path_meshes.add(path_mesh));
            commands.entity(child).despawn_recursive();
        }
    }
}

fn transform_mesh(mesh: &Mesh, transform: Transform) -> Mesh {
    let mut transformed_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let transform_attribute = |attribute: MeshVertexAttribute| {
        let values: Vec<[f32; 3]> = get_vectors(mesh, attribute.clone())
            .map(|vec3| transform.transform_point(vec3).into())
            .collect();
        transformed_mesh.insert_attribute(attribute, values);
    };
    [Mesh::ATTRIBUTE_POSITION, Mesh::ATTRIBUTE_NORMAL].map(transform_attribute);
    transformed_mesh
}

fn get_vectors(
    mesh: &Mesh,
    id: impl Into<MeshVertexAttributeId>,
) -> impl Iterator<Item = Vec3> + '_ {
    let vectors = match mesh.attribute(id).unwrap() {
        VertexAttributeValues::Float32x3(values) => values,
        // Guaranteed by Bevy
        _ => unreachable!(),
    };
    vectors.into_iter().cloned().map(Vec3::from)
}

fn get_global_transform(
    current_entity: Entity,
    parents: &Query<&Parent>,
    transforms: &Query<&Transform>,
) -> Transform {
    let own_transform = *transforms.get(current_entity).unwrap();
    match parents.get(current_entity).map(|parent| parent.get()) {
        Err(_) => own_transform,
        Ok(parent) => {
            let parent_transform = get_global_transform(parent, parents, transforms);
            parent_transform.mul_transform(own_transform)
        }
    }
}

fn get_mesh<'a>(
    children: &'a Children,
    meshes: &'a Assets<Mesh>,
    mesh_handles: &'a Query<&Handle<Mesh>>,
) -> (Entity, &'a Mesh) {
    let entity_handles: Vec<_> = children
        .iter()
        .filter_map(|entity| mesh_handles.get(*entity).ok().map(|mesh| (*entity, mesh)))
        .collect();
    assert_eq!(
        entity_handles.len(),
        1,
        "Collider must contain exactly one mesh, but found {}",
        entity_handles.len()
    );
    let (entity, mesh_handle) = entity_handles.first().unwrap();
    let mesh = meshes.get(mesh_handle).unwrap();
    assert_eq!(mesh.primitive_topology(), PrimitiveTopology::TriangleList);
    (*entity, mesh)
}
