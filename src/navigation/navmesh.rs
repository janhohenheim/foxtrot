use crate::mesh_util::get_mesh;
use bevy::prelude::*;
use bevy::render::mesh::{
    MeshVertexAttribute, MeshVertexAttributeId, PrimitiveTopology, VertexAttributeValues,
};
use bevy_pathmesh::PathMesh;

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
                mesh.set_delta(10.);
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
