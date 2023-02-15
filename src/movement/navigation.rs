use crate::movement::general_movement::Walking;
use crate::movement::navigation::navmesh::read_navmesh;
use crate::player_control::player_embodiment::Player;
use crate::util::log_error::log_errors;
use crate::util::trait_extension::{F32Ext, Vec3Ext};
use crate::GameState;
use anyhow::Context;
use anyhow::Result;
use bevy::prelude::*;
use bevy::transform::TransformSystem;
use bevy_pathmesh::PathMesh;
use bevy_pathmesh::PathMeshPlugin;
use bevy_rapier3d::prelude::*;
use serde::{Deserialize, Serialize};

pub mod navmesh;

/// Handles NPC pathfinding. Currently, all entities with the [`Follower`] component will follow the [`Player`].
/// Currently only one navmesh is supported. It is loaded automagically from any entity whose name contains `"[navmesh]"`.
pub struct NavigationPlugin;

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PathMeshPlugin)
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(query_mesh.pipe(log_errors))
                    .with_system(read_navmesh),
            )
            // See <https://bevy-cheatbook.github.io/features/transforms.html#transform-propagation>
            .add_system_to_stage(
                CoreStage::PostUpdate,
                read_navmesh.after(TransformSystem::TransformPropagate),
            );
    }
}

#[derive(Debug, Component, Clone, PartialEq, Default, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Follower;

#[allow(clippy::type_complexity)]
fn query_mesh(
    mut with_follower: Query<
        (
            Entity,
            &GlobalTransform,
            &KinematicCharacterController,
            &mut Walking,
        ),
        (With<Follower>, Without<Player>),
    >,
    with_player: Query<(Entity, &GlobalTransform), (With<Player>, Without<Follower>)>,
    path_meshes: Res<Assets<PathMesh>>,
    nav_meshes: Query<&Handle<PathMesh>>,
    rapier_context: Res<RapierContext>,
) -> Result<()> {
    for path_mesh_handle in nav_meshes.iter() {
        for (follower_entity, follower_transform, controller, mut walking) in &mut with_follower {
            for (player_entity, player_transform) in &with_player {
                let path_mesh = path_meshes
                    .get(path_mesh_handle)
                    .context("Failed to get path mesh from handle")?;
                let from = follower_transform.translation();
                let to = player_transform.translation();
                if (to - from).length_squared() < 3.0f32.squared() {
                    continue;
                }
                let max_toi = 50.;
                let solid = false;
                let filter = QueryFilter::new()
                    .exclude_sensors()
                    .exclude_collider(follower_entity);
                let path = if let
                    Some((entity, _toi)) = rapier_context.cast_ray(from, to - from, max_toi, solid, filter)
                    && entity == player_entity
                {
                    Some(to)
                } else if let Some(path) = path_mesh.transformed_path(from, to) {
                    Some(path.path[0])
                } else {
                    None
                };
                if let Some(path) = path {
                    let dir = (path - from)
                        .split(controller.up)
                        .horizontal
                        .try_normalize()
                        .context("Failed to normalize direction vector for navigation")?;
                    walking.direction = Some(dir);
                }
            }
        }
    }
    Ok(())
}
