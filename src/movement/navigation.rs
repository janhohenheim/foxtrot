use crate::level_instantiation::spawning::objects::npc;
use crate::movement::general_movement::{GeneralMovementSystemSet, Walking};
use crate::player_control::player_embodiment::Player;
use crate::util::trait_extension::{F32Ext, Vec3Ext};
use crate::GameState;

use anyhow::Result;
use bevy::prelude::*;
use bevy_mod_sysfail::*;
use bevy_rapier3d::prelude::Collider;
use oxidized_navigation::{
    query::{find_polygon_path, perform_string_pulling_on_path},
    NavMesh, NavMeshSettings, OxidizedNavigationPlugin,
};

use serde::{Deserialize, Serialize};

/// Manually tweaked
const CELL_WIDTH: f32 = 0.4 * npc::RADIUS;

/// Handles NPC pathfinding. Currently, all entities with the [`Follower`] component will follow the [`Player`].
pub(crate) fn navigation_plugin(app: &mut App) {
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
    #[cfg(feature = "dev")] _editor_state: Res<bevy_editor_pls::editor::Editor>,
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

                if let Ok(path) =
                    find_polygon_path(&nav_mesh, &nav_mesh_settings, from, to, None, None)
                {
                    let path = perform_string_pulling_on_path(&nav_mesh, from, to, &path)
                        .map_err(|e| anyhow::Error::msg(format!("{e:?}")))?;
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
