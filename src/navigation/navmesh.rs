use crate::mesh_util::{get_mesh, Meshtools};
use bevy::prelude::*;
use bevy_pathmesh::PathMesh;

#[allow(clippy::too_many_arguments)]
pub fn read_navmesh(
    mut commands: Commands,
    added_name: Query<(Entity, &Name, &Children), Added<Name>>,
    parents: Query<&Parent>,
    transforms: Query<&Transform>,
    #[cfg(debug_assertions)] mut meshes: ResMut<Assets<Mesh>>,
    #[cfg(debug_assertions)] mut materials: ResMut<Assets<StandardMaterial>>,
    #[cfg(not(debug_assertions))] meshes: Res<Assets<Mesh>>,
    mesh_handles: Query<&Handle<Mesh>>,
    mut path_meshes: ResMut<Assets<PathMesh>>,
) {
    for (parent, name, children) in &added_name {
        if name.to_lowercase().contains("[navmesh]") {
            // Necessary because at this stage the `GlobalTransform` is still at `default()` for some reason
            let global_transform = get_global_transform(parent, &parents, &transforms);
            info!("global_transform: {:?}", global_transform);
            let (child, mesh) = get_mesh(children, &meshes, &mesh_handles);
            let mesh = mesh.transformed(global_transform);

            info!("mesh: {:?}", mesh);
            let path_mesh = PathMesh::from_bevy_mesh_and_then(&mesh, |mesh| {
                mesh.set_delta(1.);
            });
            #[cfg(debug_assertions)]
            {
                let debug_mesh = path_mesh.to_mesh();
                info!("debug_mesh: {:?}", debug_mesh);
                commands.spawn((
                    PbrBundle {
                        mesh: meshes.add(debug_mesh),
                        material: materials.add(default()),
                        ..default()
                    },
                    Name::new("navmesh"),
                ));
            }
            commands.entity(parent).insert(path_meshes.add(path_mesh));
            commands.entity(child).despawn_recursive();
        }
    }
}

fn get_global_transform(
    current_entity: Entity,
    parents: &Query<&Parent>,
    transforms: &Query<&Transform>,
) -> Transform {
    let own_transform = *transforms.get(current_entity).unwrap();
    let mut transform = match parents.get(current_entity).map(|parent| parent.get()) {
        Err(_) => own_transform,
        Ok(parent) => {
            let parent_transform = get_global_transform(parent, parents, transforms);
            parent_transform.mul_transform(own_transform)
        }
    };
    transform.scale.y = 1.0;
    transform
}
