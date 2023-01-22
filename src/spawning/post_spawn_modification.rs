use crate::shader::Materials;
use bevy::prelude::*;
use bevy::render::mesh::{
    MeshVertexAttribute, MeshVertexAttributeId, PrimitiveTopology, VertexAttributeValues,
};
use bevy::utils::HashSet;
use bevy_pathmesh::PathMesh;
use bevy_rapier3d::prelude::*;
use itertools::Itertools;
use ordered_float::OrderedFloat;
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
    transformed_mesh.set_indices(mesh.indices().cloned());
    transformed_mesh
}

fn poke_faces(mesh: &Mesh, max_distance_to_centers: f32) -> Mesh {
    let max_distance_squared = max_distance_to_centers.powf(2.);
    let mut vertices: Vec<_> = get_vectors(mesh, Mesh::ATTRIBUTE_POSITION).collect();
    let mut triangles: Vec<Triangle> = mesh
        .indices()
        .unwrap()
        .iter()
        .tuples()
        .map(|(a, b, c)| [a, b, c])
        .enumerate()
        .map(|(index, vertex_indices)| Triangle(vertex_indices.map(|index| vertices[index])))
        .collect();
    let mut triangles_to_remove = Vec::new();
    let mut new_vertices = Vec::new();
    let mut new_triangles = Vec::new();
    for (index, triangle) in triangles.iter().enumerate() {
        let poked = poke_triangle(
            *triangle,
            max_distance_squared,
            &mut new_vertices,
            &mut new_triangles,
        );
        if poked {
            triangles_to_remove.push(index);
        }
    }
    for index in triangles_to_remove {
        triangles.remove(index);
    }
    let mut poked_mesh = Mesh::new(PrimitiveTopology::TriangleList);

    for triangle in new_triangles {
        triangles.push(triangle);
    }
    poked_mesh
}

fn poke_triangle(
    triangle: Triangle,
    max_distance_squared: f32,
    new_vertices: &mut Vec<Vec3>,
    new_triangles: &mut Vec<Triangle>,
) -> bool {
    let new_vertex = triangle.center();
    if triangle.max_distance_squared(new_vertex) < max_distance_squared {
        new_triangles.push(triangle);
        return false;
    }
    new_vertices.push(new_vertex);

    [
        Triangle([triangle.0[0], triangle.0[1], new_vertex]),
        Triangle([triangle.0[1], triangle.0[2], new_vertex]),
        Triangle([triangle.0[2], triangle.0[0], new_vertex]),
    ]
    .map(|triangle| poke_triangle(triangle, max_distance_squared, new_vertices, new_triangles))
    .contains(&true)
}

#[derive(Debug, Clone, Copy)]
struct Triangle([Vec3; 3]);

impl Triangle {
    fn center(self) -> Vec3 {
        self.0.into_iter().sum::<Vec3>() / 3.
    }

    fn max_distance_squared(self, center: Vec3) -> f32 {
        *self
            .0
            .map(|coords| (coords - center).length_squared())
            .map(OrderedFloat)
            .into_iter()
            .max()
            .unwrap()
    }
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
