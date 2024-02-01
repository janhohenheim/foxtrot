use crate::level_instantiation::spawning::objects::CollisionLayer;
use crate::util::trait_extension::MeshExt;
use crate::GameState;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy_mod_sysfail::*;
use bevy_xpbd_3d::prelude::*;
use oxidized_navigation::NavMeshAffector;

/// Sets up the [`RapierPhysicsPlugin`] and [`RapierConfiguration`].
pub(crate) fn physics_plugin(app: &mut App) {
    app.add_plugins(PhysicsPlugins::default())
        .add_systems(Update, read_colliders.run_if(in_state(GameState::Playing)));
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
            let mesh = Mesh::find_mesh(entity, &children, &meshes, &mesh_handles)
                .context("Failed to find mesh for collider")?;
            let collider =
                Collider::trimesh_from_mesh(mesh).context("Failed to create collider from mesh")?;

            commands.entity(entity).insert((
                collider,
                RigidBody::Static,
                CollisionLayers::new([CollisionLayer::Solid], [CollisionLayer::Solid]),
                NavMeshAffector,
            ));
        }
    }
    Ok(())
}
