use crate::mesh_util::get_mesh;
use bevy::prelude::*;
use bevy::render::mesh::{
    Indices, MeshVertexAttribute, MeshVertexAttributeId, PrimitiveTopology, VertexAttributeValues,
};
use bevy_pathmesh::PathMesh;
use itertools::Itertools;
use ordered_float::OrderedFloat;

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
            let delta = 1.;
            let mesh = poke_faces(&mesh, delta);
            let path_mesh = PathMesh::from_bevy_mesh_and_then(&mesh, |mesh| {
                mesh.set_delta(delta);
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
    let mut normals: Vec<_> = get_vectors(mesh, Mesh::ATTRIBUTE_NORMAL).collect();
    let mut triangles: Vec<Triangle> = mesh
        .indices()
        .unwrap()
        .iter()
        .tuples()
        .map(|(a, b, c)| [a, b, c])
        .map(|vertex_indices| {
            Triangle(vertex_indices.map(|index| Vertex {
                index,
                coords: vertices[index],
            }))
        })
        .collect();
    let mut triangles_to_remove = Vec::new();
    let mut new_triangles = Vec::new();
    for (index, triangle) in triangles.iter().enumerate() {
        let poked = poke_triangle(
            *triangle,
            max_distance_squared,
            &mut vertices,
            &mut normals,
            &mut new_triangles,
        );
        if poked {
            triangles_to_remove.push(index);
        }
    }

    for index in triangles_to_remove {
        triangles.remove(index);
    }
    for triangle in new_triangles {
        triangles.push(triangle);
    }

    let mut poked_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let indices: Vec<_> = triangles
        .into_iter()
        .map(|triangle| triangle.0.map(|vertex| vertex.index as u32))
        .flatten()
        .collect();
    poked_mesh.set_indices(Some(Indices::U32(indices)));
    let vertices: Vec<_> = vertices.iter().cloned().map(<[f32; 3]>::from).collect();
    poked_mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    let normals: Vec<_> = normals.into_iter().map(<[f32; 3]>::from).collect();
    poked_mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);

    poked_mesh
}

fn poke_triangle(
    triangle: Triangle,
    max_distance_squared: f32,
    vertices: &mut Vec<Vec3>,
    normals: &mut Vec<Vec3>,
    new_triangles: &mut Vec<Triangle>,
) -> bool {
    let center = triangle.center();
    if triangle.max_distance_squared(center) < max_distance_squared {
        new_triangles.push(triangle);
        return false;
    }
    let normal = triangle
        .0
        .map(|vertex| normals[vertex.index])
        .iter()
        .sum::<Vec3>()
        / 3.;
    normals.push(normal);
    vertices.push(center);
    let new_vertex = Vertex {
        index: vertices.len() - 1,
        coords: center,
    };

    [
        Triangle([triangle.0[0], triangle.0[1], new_vertex]),
        Triangle([triangle.0[1], triangle.0[2], new_vertex]),
        Triangle([triangle.0[2], triangle.0[0], new_vertex]),
    ]
    .map(|triangle| {
        poke_triangle(
            triangle,
            max_distance_squared,
            vertices,
            normals,
            new_triangles,
        )
    })
    .contains(&true)
}

#[derive(Debug, Clone, Copy)]
struct Triangle([Vertex; 3]);

impl Triangle {
    fn center(self) -> Vec3 {
        self.0.into_iter().map(|vertex| vertex.coords).sum::<Vec3>() / 3.
    }

    fn max_distance_squared(self, center: Vec3) -> f32 {
        *self
            .0
            .map(|vertex| (vertex.coords - center).length_squared())
            .map(OrderedFloat)
            .into_iter()
            .max()
            .unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
struct Vertex {
    index: usize,
    coords: Vec3,
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
