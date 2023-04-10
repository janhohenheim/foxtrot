use crate::util::trait_extension::MeshExt;
use bevy::prelude::*;
use bevy_pathmesh::PathMesh;
#[cfg(feature = "dev")]
use serde::{Deserialize, Serialize};

pub(crate) fn read_navmesh(
    mut commands: Commands,
    added_name: Query<(Entity, &Name, &GlobalTransform), Added<Name>>,
    children: Query<&Children>,
    meshes: Res<Assets<Mesh>>,
    mesh_handles: Query<&Handle<Mesh>>,
    mut path_meshes: ResMut<Assets<PathMesh>>,
) {
    #[cfg(feature = "tracing")]
    let _span = info_span!("read_navmesh").entered();
    for (parent, name, global_transform) in &added_name {
        if name.to_lowercase().contains("[navmesh]") {
            let transform = global_transform.compute_transform();
            for (_child, mesh) in
                Mesh::search_in_children(parent, &children, &meshes, &mesh_handles)
            {
                let mesh = mesh.transformed(transform);

                let path_mesh = PathMesh::from_bevy_mesh_and_then(&mesh, |mesh| {
                    mesh.set_delta(5.);
                });
                commands.entity(parent).insert((
                    path_meshes.add(path_mesh),
                    #[cfg(feature = "dev")]
                    NavMesh,
                ));
            }
        }
    }
}

#[cfg(feature = "dev")]
#[derive(Debug, Component, Clone, PartialEq, Default, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub(crate) struct NavMesh;
