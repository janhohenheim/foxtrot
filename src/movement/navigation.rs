#[cfg(feature = "dev")]
use crate::dev::dev_editor::DevEditorWindow;
use crate::{
    level_instantiation::on_spawn::{player, Npc, Player},
    movement::character_controller::{GeneralMovementSystemSet, Walk},
    util::math_trait_ext::{F32Ext, Vec3Ext},
    GameState,
};
#[cfg(feature = "dev")]
use anyhow::Context;
use bevy::prelude::*;
use bevy_mod_sysfail::prelude::*;
use bevy_xpbd_3d::prelude::Collider;
#[cfg(feature = "dev")]
use oxidized_navigation::debug_draw::{DrawNavMesh, DrawPath, OxidizedNavigationDebugDrawPlugin};
use oxidized_navigation::{
    query::{find_polygon_path, perform_string_pulling_on_path},
    NavMesh, NavMeshSettings, OxidizedNavigationPlugin,
};

/// Manually tweaked
const CELL_WIDTH: f32 = 0.4 * player::RADIUS;

/// Handles NPC pathfinding. Currently, all entities with the [`Npc`] component will follow the [`Player`].
pub(super) fn plugin(app: &mut App) {
    // consts manually tweaked
    app.add_plugins(OxidizedNavigationPlugin::<Collider>::new(NavMeshSettings {
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
        max_tile_generation_tasks: None,
    }))
    .add_systems(
        Update,
        query_mesh
            .before(GeneralMovementSystemSet)
            .run_if(in_state(GameState::Playing)),
    );
    #[cfg(feature = "dev")]
    app.add_plugins(OxidizedNavigationDebugDrawPlugin)
        .add_systems(Update, draw_navmesh);
}

#[sysfail(Log<anyhow::Error, Error>)]
fn query_mesh(
    #[cfg(feature = "dev")] mut commands: Commands,
    mut with_follower: Query<(&Transform, &mut Walk), (With<Npc>, Without<Player>)>,
    with_player: Query<&Transform, (With<Player>, Without<Npc>)>,
    nav_mesh_settings: Res<NavMeshSettings>,
    nav_mesh: Res<NavMesh>,
    #[cfg(feature = "dev")] editor_state: Res<bevy_editor_pls::editor::Editor>,
) {
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

                if let Ok(path) =
                    find_polygon_path(&nav_mesh, &nav_mesh_settings, from, to, None, None)
                {
                    let path = perform_string_pulling_on_path(&nav_mesh, from, to, &path)
                        .map_err(|e| anyhow::Error::msg(format!("{e:?}")))?;
                    #[cfg(feature = "dev")]
                    {
                        let nav_render_enabled = editor_state
                            .window_state::<DevEditorWindow>()
                            .context("Failed to read dev window state")?
                            .navmesh_render_enabled;
                        if nav_render_enabled {
                            let shifted_path = path
                                .iter()
                                .map(|point| *point + Vec3::new(0., 0.2, 0.))
                                .collect::<Vec<_>>();
                            commands.spawn(DrawPath {
                                timer: Some(Timer::from_seconds(4.0, TimerMode::Once)),
                                pulled_path: shifted_path,
                                color: Color::BLUE,
                            });
                        }
                    }
                    let dir = path
                        .into_iter()
                        .map(|next_point| (next_point - from).horizontal())
                        .filter(|dir| dir.length_squared() > 1e-3f32.squared())
                        .filter_map(|dir| dir.try_normalize())
                        .next();
                    walking.direction = dir;
                }
            }
        }
    }
}

#[cfg(feature = "dev")]
#[sysfail(Log<anyhow::Error, Error>)]
fn draw_navmesh(
    editor: Res<bevy_editor_pls::editor::Editor>,
    mut draw_nav_mesh: ResMut<DrawNavMesh>,
) {
    let nav_render_enabled = editor
        .window_state::<DevEditorWindow>()
        .context("Failed to read dev window state")?
        .navmesh_render_enabled;
    draw_nav_mesh.0 = nav_render_enabled;
}
