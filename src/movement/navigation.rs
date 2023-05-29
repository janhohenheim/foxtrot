#[cfg(feature = "dev")]
use crate::dev::dev_editor::DevEditorWindow;
use crate::level_instantiation::spawning::objects::npc;
use crate::movement::general_movement::{GeneralMovementSystemSet, Walking};
use crate::player_control::player_embodiment::Player;
use crate::util::trait_extension::{F32Ext, Vec3Ext};
use crate::GameState;
#[cfg(feature = "dev")]
use anyhow::Context;
use anyhow::Result;
use bevy::prelude::*;
use bevy_mod_sysfail::macros::*;
#[cfg(feature = "dev")]
use bevy_prototype_debug_lines::DebugLines;
use oxidized_navigation::{
    query::{find_path, perform_string_pulling_on_path},
    NavMesh, NavMeshSettings, OxidizedNavigationPlugin,
};
use serde::{Deserialize, Serialize};

/// Manually tweaked
const CELL_WIDTH: f32 = 0.4 * npc::RADIUS;

/// Handles NPC pathfinding. Currently, all entities with the [`Follower`] component will follow the [`Player`].
pub(crate) fn navigation_plugin(app: &mut App) {
    app.add_plugin(OxidizedNavigationPlugin)
        // consts manually tweaked
        .insert_resource(NavMeshSettings {
            cell_width: CELL_WIDTH,
            cell_height: 0.5 * CELL_WIDTH,
            tile_width: 170,
            world_half_extents: 250.0,
            world_bottom_bound: -20.0,
            max_traversable_slope_radians: (40.0_f32 - 0.1).to_radians(),
            walkable_height: 25,
            walkable_radius: 4,
            step_height: 3,
            min_region_area: 30,
            merge_region_area: 500,
            max_contour_simplification_error: 1.3,
            max_edge_length: 100,
        })
        .add_system(
            query_mesh
                .before(GeneralMovementSystemSet)
                .in_set(OnUpdate(GameState::Playing)),
        );
}

#[derive(Debug, Component, Clone, PartialEq, Default, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub(crate) struct Follower;

#[sysfail(log(level = "error"))]
fn query_mesh(
    mut with_follower: Query<(&Transform, &mut Walking), (With<Follower>, Without<Player>)>,
    with_player: Query<&Transform, (With<Player>, Without<Follower>)>,
    nav_mesh_settings: Res<NavMeshSettings>,
    nav_mesh: Res<NavMesh>,
    #[cfg(feature = "dev")] mut lines: ResMut<DebugLines>,
    #[cfg(feature = "dev")] editor_state: Res<bevy_editor_pls::editor::Editor>,
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
