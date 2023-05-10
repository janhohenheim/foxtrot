use crate::util::trait_extension::MeshExt;
use crate::GameState;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy_mod_sysfail::macros::*;
use bevy_rapier3d::prelude::*;
use oxidized_navigation::NavMeshAffector;

/// Sets up the [`RapierPhysicsPlugin`] and [`RapierConfiguration`].
pub(crate) fn physics_plugin(app: &mut App) {
    app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .insert_resource(RapierConfiguration {
            timestep_mode: TimestepMode::Variable {
                max_dt: 1.0 / 20.0,
                time_scale: 1.0,
                substeps: 4,
            },
            ..default()
        })
        .add_system(read_colliders.in_set(OnUpdate(GameState::Playing)));
}

#[sysfail(log(level = "error"))]
pub(crate) fn read_colliders(
    mut commands: Commands,
    added_name: Query<(Entity, &Name), Added<Name>>,
    children: Query<&Children>,
    meshes: Res<Assets<Mesh>>,
    mesh_handles: Query<&Handle<Mesh>>,
) -> Result<()> {
    #[cfg(feature = "tracing")]
    let _span = info_span!("read_colliders").entered();
    for (entity, name) in &added_name {
        if name.to_lowercase().contains("[collider]") {
            for (collider_entity, collider_mesh) in
                Mesh::search_in_children(entity, &children, &meshes, &mesh_handles)
            {
                let rapier_collider =
                    Collider::from_bevy_mesh(collider_mesh, &ComputedColliderShape::TriMesh)
                        .context("Failed to create collider from mesh")?;

                commands
                    .entity(collider_entity)
                    .insert((rapier_collider, NavMeshAffector));
            }
        }
    }
    Ok(())
}
