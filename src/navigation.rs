use crate::player::{CharacterVelocity, Player};
use crate::spawning::objects::{npc, player};
use crate::GameState;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_pathmesh::PathMesh;
use bevy_pathmesh::PathmeshPlugin;
use bevy_prototype_debug_lines::DebugLines;
use oxidized_navigation::{
    query::{find_path, perform_string_pulling_on_path},
    NavMesh, NavMeshSettings, OxidizedNavigationPlugin,
};
use serde::{Deserialize, Serialize};
use std::iter;

pub struct NavigationPlugin;

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(OxidizedNavigationPlugin)
            .add_plugin(PathmeshPlugin)
            .insert_resource(NavMeshSettings {
                cell_width: 0.4,
                cell_height: 0.1,
                tile_width: 200,
                world_half_extents: 230.0,
                world_bottom_bound: -10.0,
                max_traversable_slope_radians: (40.0_f32 - 0.1).to_radians(),
                walkable_height: 20,
                walkable_radius: 2,
                step_height: 3,
                min_region_area: 100,
                merge_region_area: 500,
                max_contour_simplification_error: 0.5,
                max_edge_length: 50,
            })
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(query_mesh));
    }
}

#[derive(Debug, Component, Clone, PartialEq, Default, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Follower;

#[allow(clippy::type_complexity)]
fn query_mesh(
    time: Res<Time>,
    mut with_follower: Query<
        (&GlobalTransform, &mut CharacterVelocity),
        (With<Follower>, Without<Player>),
    >,
    with_player: Query<&GlobalTransform, (With<Player>, Without<Follower>)>,
    mut lines: ResMut<DebugLines>,
    path_meshes: Res<Assets<PathMesh>>,
    nav_meshes: Query<(&GlobalTransform, &Handle<PathMesh>), (Without<Follower>, Without<Player>)>,
) {
    let dt = time.delta_seconds();
    for (mesh_transform, path_mesh_handle) in nav_meshes.iter() {
        for (follower_transform, mut character_velocity) in &mut with_follower {
            for player_transform in &with_player {
                let path_mesh = path_meshes.get(path_mesh_handle).unwrap();
                let path = path_mesh.path(
                    follower_transform.translation().xz(),
                    player_transform.translation().xz(),
                );
                let transform = mesh_transform.compute_transform();
                let inverse_transform = Transform {
                    translation: -transform.translation,
                    rotation: transform.rotation.inverse(),
                    scale: 1. / transform.scale,
                };
                info!(
                    "{:?}",
                    inverse_transform.transform_point(follower_transform.translation())
                );
                // info!("{:?}", path);
            }
        }
    }
}
