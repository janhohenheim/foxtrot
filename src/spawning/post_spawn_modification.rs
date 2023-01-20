use crate::shader::Materials;
use bevy::prelude::*;
use bevy::render::mesh::{PrimitiveTopology, VertexAttributeValues};
use bevy_pathmesh::PathMesh;
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
        if name.to_lowercase().contains("[collider]") {
            let (collider_entity, collider_mesh) = get_mesh(children, &meshes, &mesh_handles);
            commands.entity(collider_entity).despawn_recursive();

            let rapier_collider =
                Collider::from_bevy_mesh(collider_mesh, &ComputedColliderShape::TriMesh).unwrap();

            commands
                .entity(entity)
                .insert((rapier_collider, NavMeshAffector::default()));
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
    added_name: Query<(&Name, &Children), Added<Name>>,
    meshes: Res<Assets<Mesh>>,
    mesh_handles: Query<&Handle<Mesh>>,
    path_meshes: Res<Assets<PathMesh>>,
) {
    for (name, children) in &added_name {
        if name.to_lowercase().contains("[navmesh]") {
            let (entity, mesh) = get_mesh(children, &meshes, &mesh_handles);

            let mesh_vertices = match mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap() {
                VertexAttributeValues::Float32x3(values) => values,
                _ => panic!(),
            };

            let triangle_edge_indices = mesh.indices().unwrap();
            let polygons: Vec<_> = triangle_edge_indices
                .iter()
                .zip(
                    triangle_edge_indices
                        .iter()
                        .skip(1)
                        .zip(triangle_edge_indices.iter().skip(2)),
                )
                .map(|(a, (b, c))| {
                    let vertices = [a, b, c].map(|index| index.try_into().unwrap()).to_vec();
                    let is_one_way = false;
                    polyanya::Polygon::new(vertices, is_one_way)
                })
                .collect();

            let vertices: Vec<_> = mesh_vertices
                .iter()
                // Drop the y coord
                .map(|coords| Vec2::new(coords[0], coords[2]))
                .enumerate()
                .map(|(vertex_index, coords)| {
                    let neighbors = polygons
                        .iter()
                        .enumerate()
                        .filter_map(|(polygon_index, polygon)| {
                            polygon
                                .vertices
                                .contains(&(vertex_index as u32))
                                .then(|| polygon_index)
                        })
                        .map(|index| isize::try_from(index).unwrap())
                        .collect();
                    polyanya::Vertex::new(coords, neighbors)
                })
                .collect();
            info!("vertices: {:?}", vertices);
            info!("polygons: {:?}", polygons);
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn get_mesh<'a>(
    children: &'a Children,
    meshes: &'a Res<'a, Assets<Mesh>>,
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
