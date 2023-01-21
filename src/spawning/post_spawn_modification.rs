use crate::shader::Materials;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy::render::mesh::{PrimitiveTopology, VertexAttributeValues};
use bevy_pathmesh::PathMesh;
use bevy_rapier3d::prelude::*;
use itertools::Itertools;
use ordered_float::OrderedFloat;
use polyanya::Vertex;
use serde::{Deserialize, Serialize};
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
        if name.to_lowercase().contains("[ground]") && false {
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
            let mesh_vertices = match mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap() {
                VertexAttributeValues::Float32x3(values) => values,
                _ => panic!(),
            };

            let triangle_edge_indices = mesh.indices().unwrap();
            let triangles: Vec<_> = triangle_edge_indices
                .iter()
                .tuples()
                .map(|(a, b, c)| [a, b, c].map(|index| index.try_into().unwrap()).to_vec())
                .collect();

            let mut vertices: Vec<_> = mesh_vertices
                .into_iter()
                .map(|coords| (*coords).into())
                .map(|coords| global_transform.transform_point(coords))
                .map(|coords| coords.xz())
                .enumerate()
                .map(|(vertex_index, coords)| {
                    let neighbor_indices = triangles
                        .iter()
                        .enumerate()
                        .filter_map(|(polygon_index, vertex_indices_in_polygon)| {
                            vertex_indices_in_polygon
                                .contains(&(vertex_index as u32))
                                .then_some(polygon_index)
                        })
                        .map(|index| isize::try_from(index).unwrap())
                        .collect();
                    polyanya::Vertex::new(coords, neighbor_indices)
                })
                .collect();
            let polygons: Vec<_> = triangles
                .into_iter()
                .map(|vertex_indices_in_polygon| {
                    let is_one_way = vertex_indices_in_polygon
                        .iter()
                        .map(|index| &vertices[*index as usize])
                        .map(|vertex| &vertex.polygons)
                        .flatten()
                        .unique()
                        .take(3)
                        .count()
                        // One way means all vertices have at most 2 neighbors: the original polygon and one other
                        < 3;
                    polyanya::Polygon::new(vertex_indices_in_polygon, is_one_way)
                })
                .collect();
            let unordered_vertices = vertices.clone();
            for (vertex_index, vertex) in vertices.iter_mut().enumerate() {
                vertex.polygons.sort_by_key(|index| {
                    // No -1 present yet, so the unwrap is safe
                    let index = usize::try_from(*index).unwrap();
                    let polygon = &polygons[index];
                    let unrolled_indices = polygon.unroll_at(vertex_index).unwrap();
                    let triangle_center_direction: Vec2 =
                        [unrolled_indices.0, unrolled_indices.1, unrolled_indices.2]
                            .into_iter()
                            .map(|index| &unordered_vertices[index])
                            .map(|vertex| vertex.coords)
                            .sum();
                    let angle_to_positive_x_axis = triangle_center_direction
                        .y
                        .atan2(triangle_center_direction.x);
                    OrderedFloat(angle_to_positive_x_axis)
                });
                let mut polygons_including_obstacles = vec![vertex.polygons[0]];
                for polygon_index in vertex
                    .polygons
                    .iter()
                    .cloned()
                    .skip(1)
                    .chain(iter::once(polygons_including_obstacles[0]))
                {
                    let last_index = *polygons_including_obstacles.last().unwrap();
                    if last_index == -1 {
                        polygons_including_obstacles.push(polygon_index);
                        continue;
                    }
                    let last_polygon = &polygons[usize::try_from(last_index).unwrap()];
                    let last_counterclockwise_neighbor = last_polygon
                        .unroll_at(vertex_index)
                        .unwrap()
                        .get_counterclockwise_neighbor(&unordered_vertices);

                    let next_polygon = &polygons[usize::try_from(polygon_index).unwrap()];
                    let next_clockwise_neighbor = next_polygon
                        .unroll_at(vertex_index)
                        .unwrap()
                        .get_clockwise_neighbor(&unordered_vertices);
                    if last_counterclockwise_neighbor != next_clockwise_neighbor {
                        // The edges don't align; there's an obstacle here
                        polygons_including_obstacles.push(-1);
                    }
                    polygons_including_obstacles.push(polygon_index);
                }
                // The first element is included in the end again
                polygons_including_obstacles.remove(0);
                vertex.polygons = polygons_including_obstacles;
            }
            // Recreate vertices because we now include obstacles, so vertices can now be properly identified as edges
            let vertices: Vec<_> = vertices
                .into_iter()
                .map(|vertex| polyanya::Vertex::new(vertex.coords, vertex.polygons))
                .collect();
            info!("vertices: {:?}", vertices);
            info!("polygons: {:?}", polygons);
            let mut polyanya_mesh = polyanya::Mesh::new(vertices, polygons);
            info!("polyanya_mesh: {:?}", polyanya_mesh);
            polyanya_mesh.bake();
            let path_mesh = PathMesh::from_polyanya_mesh(polyanya_mesh);

            commands.entity(child).despawn_recursive();
            commands.entity(parent).insert(path_meshes.add(path_mesh));
        }
    }
}

trait PolygonExtension {
    fn do_edges_on_corner_align(&self, other: &Self, vertex: usize) -> Option<bool>;
    fn unroll_at(&self, vertex_index: usize) -> Option<(usize, usize, usize)>;
}
impl PolygonExtension for polyanya::Polygon {
    fn do_edges_on_corner_align(&self, other: &Self, vertex: usize) -> Option<bool> {
        let unrolled_self = self.unroll_at(vertex)?;
        let unrolled_other = other.unroll_at(vertex)?;
        Some(unrolled_self.0 == unrolled_other.2 || unrolled_self.2 == unrolled_other.0)
    }

    fn unroll_at(&self, vertex_index: usize) -> Option<(usize, usize, usize)> {
        unroll(self).find(|(_a, b, _c)| *b == vertex_index)
    }
}

trait UnrolledExtension {
    fn get_sorted_counterclockwise(&self, vertices: &[Vertex]) -> (usize, usize);
    fn get_counterclockwise_neighbor(&self, vertices: &[Vertex]) -> usize;
    fn get_clockwise_neighbor(&self, vertices: &[Vertex]) -> usize;
}
impl UnrolledExtension for (usize, usize, usize) {
    fn get_sorted_counterclockwise(&self, vertices: &[Vertex]) -> (usize, usize) {
        let mut other_indices = [self.0, self.2];
        let own_coords = vertices[self.1].coords;
        other_indices.sort_by_key(|index| {
            let dir = vertices[*index].coords - own_coords;
            OrderedFloat(dir.y.atan2(dir.x))
        });
        (other_indices[0], other_indices[1])
    }
    fn get_counterclockwise_neighbor(&self, vertices: &[Vertex]) -> usize {
        self.get_sorted_counterclockwise(vertices).0
    }

    fn get_clockwise_neighbor(&self, vertices: &[Vertex]) -> usize {
        self.get_sorted_counterclockwise(vertices).1
    }
}

fn unroll(polygon: &polyanya::Polygon) -> impl Iterator<Item = (usize, usize, usize)> + '_ {
    polygon
        .vertices
        .iter()
        .chain(polygon.vertices.iter().take(2))
        .tuple_windows()
        .map(|(a, b, c)| (*a as usize, *b as usize, *c as usize))
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
