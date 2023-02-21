use crate::util::log_error::log_errors;
use crate::util::trait_extension::MeshExt;
use crate::GameState;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use oxidized_navigation::NavMeshAffector;

/// Sets up the [`RapierPhysicsPlugin`] and [`RapierConfiguration`].
pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
            .insert_resource(RapierConfiguration {
                timestep_mode: TimestepMode::Variable {
                    max_dt: 1.0 / 20.0,
                    time_scale: 1.0,
                    substeps: 1,
                },
                ..default()
            })
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(read_colliders.pipe(log_errors)),
            );
    }
}

pub fn read_colliders(
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
                    .insert((rapier_collider, NavMeshAffector::default()));
            }
        }
    }
    Ok(())
}
