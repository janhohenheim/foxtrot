#[cfg(feature = "dev")]
use crate::dev::dev_editor::DevEditorWindow;
use crate::level_instantiation::spawning::objects::npc;
use crate::movement::general_movement::{apply_walking, reset_movement_components, Walking};
use crate::player_control::player_embodiment::Player;
use crate::util::log_error::log_errors;
use crate::util::trait_extension::{F32Ext, Vec3Ext};
use crate::GameState;
#[cfg(feature = "dev")]
use anyhow::Context;
use anyhow::Result;
use bevy::prelude::*;
#[cfg(feature = "dev")]
use bevy_prototype_debug_lines::DebugLines;
use oxidized_navigation::{
    query::{find_path, perform_string_pulling_on_path},
    NavMesh, NavMeshGenerationState, NavMeshSettings, OxidizedNavigationPlugin,
};
use serde::{Deserialize, Serialize};

/// Handles NPC pathfinding. Currently, all entities with the [`Follower`] component will follow the [`Player`].
/// Currently only one navmesh is supported. It is loaded automagically from any entity whose name contains `"[navmesh]"`.
pub struct NavigationPlugin;

const CELL_WIDTH: f32 = 0.5 * npc::RADIUS;

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(OxidizedNavigationPlugin {
            starting_state: NavMeshGenerationState::Running, // Generate tile updates.
        })
        .insert_resource(NavMeshSettings {
            cell_width: CELL_WIDTH,
            cell_height: 0.5 * CELL_WIDTH,
            tile_width: 150,
            world_half_extents: 250.0,
            world_bottom_bound: -100.0,
            max_traversable_slope_radians: (40.0_f32 - 0.1).to_radians(),
            walkable_height: 25,
            walkable_radius: 3,
            step_height: 3,
            min_region_area: 50,
            merge_region_area: 500,
            max_contour_simplification_error: 1.1,
            max_edge_length: 70,
        })
        .add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(
                query_mesh
                    .pipe(log_errors)
                    .after(reset_movement_components)
                    .before(apply_walking),
            ),
        );
    }
}

#[derive(Debug, Component, Clone, PartialEq, Default, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Follower;

fn query_mesh(
    mut with_follower: Query<(&Transform, &mut Walking), (With<Follower>, Without<Player>)>,
    with_player: Query<&Transform, (With<Player>, Without<Follower>)>,
    nav_mesh_settings: Res<NavMeshSettings>,
    nav_mesh: Res<NavMesh>,
    #[cfg(feature = "dev")] mut lines: ResMut<DebugLines>,
    #[cfg(feature = "dev")] editor_state: Res<bevy_editor_pls::Editor>,
) -> Result<()> {
    #[cfg(feature = "tracing")]
    let _span = info_span!("query_mesh").entered();
    if let Ok(nav_mesh) = nav_mesh.get().read() {
        for (follower_transform, mut walking) in &mut with_follower {
            for player_transform in &with_player {
                let from = follower_transform.translation;
                let to = player_transform.translation;
                if (to - from).length_squared() < 3.0f32.squared() {
                    continue;
                }

                if let Ok(path) = find_path(&nav_mesh, &nav_mesh_settings, from, to, None, None) {
                    let path = perform_string_pulling_on_path(&nav_mesh, from, to, &path)
                        .map_err(|e| anyhow::Error::msg(format!("{e:?}")))?;
                    #[cfg(feature = "dev")]
                    if editor_state
                        .window_state::<DevEditorWindow>()
                        .context("Failed to get dev window state")?
                        .navmesh_render_enabled
                    {
                        draw_path(&path, &mut lines, Color::RED);
                    }
                    let dir = path
                        .into_iter()
                        .map(|next_point| {
                            (next_point - from)
                                .split(follower_transform.up())
                                .horizontal
                        })
                        .filter(|dir| dir.length_squared() > 1e-3f32.squared())
                        .filter_map(|dir| dir.try_normalize())
                        .next();
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
