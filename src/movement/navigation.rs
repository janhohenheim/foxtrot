#[cfg(feature = "dev")]
use crate::dev::scene_editor::SceneEditorState;
use crate::movement::general_movement::Walking;
use crate::player_control::player_embodiment::Player;
use crate::util::log_error::log_errors;
use crate::util::trait_extension::{F32Ext, Vec3Ext};
use crate::GameState;
use anyhow::Result;
use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;
use bevy_rapier3d::prelude::*;
use oxidized_navigation::{
    query::{find_path, perform_string_pulling_on_path},
    NavMesh, NavMeshGenerationState, NavMeshSettings, OxidizedNavigationPlugin,
};
use serde::{Deserialize, Serialize};

/// Handles NPC pathfinding. Currently, all entities with the [`Follower`] component will follow the [`Player`].
/// Currently only one navmesh is supported. It is loaded automagically from any entity whose name contains `"[navmesh]"`.
pub struct NavigationPlugin;

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(OxidizedNavigationPlugin {
            starting_state: NavMeshGenerationState::Running, // Generate tile updates.
        })
        .insert_resource(NavMeshSettings {
            cell_width: 0.25,
            cell_height: 0.1,
            tile_width: 100,
            world_half_extents: 250.0,
            world_bottom_bound: -100.0,
            max_traversable_slope_radians: (40.0_f32 - 0.1).to_radians(),
            walkable_height: 20,
            walkable_radius: 1,
            step_height: 3,
            min_region_area: 100,
            merge_region_area: 500,
            max_contour_simplification_error: 1.1,
            max_edge_length: 80,
        })
        .add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(query_mesh.pipe(log_errors)),
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
            &Transform,
            &KinematicCharacterController,
            &mut Walking,
        ),
        (With<Follower>, Without<Player>),
    >,
    with_player: Query<(Entity, &Transform), (With<Player>, Without<Follower>)>,
    nav_mesh_settings: Res<NavMeshSettings>,
    nav_mesh: Res<NavMesh>,
    mut lines: ResMut<DebugLines>,
    rapier_context: Res<RapierContext>,
    #[cfg(feature = "dev")] editor_state: Res<SceneEditorState>,
) -> Result<()> {
    if let Ok(nav_mesh) = nav_mesh.get().read() {
        for (follower_entity, follower_transform, controller, mut walking) in &mut with_follower {
            for (player_entity, player_transform) in &with_player {
                let from = follower_transform.translation;
                let to = player_transform.translation;
                if (to - from).length_squared() < 3.0f32.squared() {
                    continue;
                }

                let max_toi = 50.;
                let solid = false;
                let filter = QueryFilter::new()
                    .exclude_sensors()
                    .exclude_collider(follower_entity);
                let path = if let Some((entity, _toi)) =
                    rapier_context.cast_ray(from, to - from, max_toi, solid, filter)
                    && entity == player_entity
                {
                    Some(vec![from, to])
                } else if let Ok(path) = find_path(
                    &nav_mesh,
                    &nav_mesh_settings,
                    from,
                    to,
                    None,
                    None,
                ) {
                    let string_path = perform_string_pulling_on_path(&nav_mesh, from, to, &path)
                        .map_err(|e| anyhow::Error::msg(format!("{e:?}")))?;
                    Some(string_path)
                } else {
                    None
                };
                if let Some(path) = path {
                    #[cfg(feature = "dev")]
                    if editor_state.navmesh_render_enabled {
                        draw_path(&path, &mut lines, Color::RED);
                    }
                    let next_point = path[1];
                    let dir = (next_point - from)
                        .split(controller.up)
                        .horizontal
                        .try_normalize();
                    walking.direction = dir;
                }
            }
        }
    }

    Ok(())
}

#[cfg(feature = "dev")]
fn draw_path(path: &[Vec3], lines: &mut DebugLines, color: Color) {
    for (a, b) in path.iter().zip(path.iter().skip(1)) {
        lines.line_colored(*a, *b, 0.1, color);
    }
}
