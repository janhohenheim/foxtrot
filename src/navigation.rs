use crate::player::{CharacterVelocity, Player};
use crate::spawning::objects::{npc, player};
use crate::GameState;
use bevy::prelude::*;
use oxidized_navigation::{
    query::{find_path, perform_string_pulling_on_path},
    NavMesh, NavMeshSettings, OxidizedNavigationPlugin,
};
use serde::{Deserialize, Serialize};

pub struct NavigationPlugin;

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(OxidizedNavigationPlugin)
            .insert_resource(NavMeshSettings {
                cell_width: npc::RADIUS * npc::SCALE * 0.5,
                cell_height: npc::RADIUS * npc::SCALE * 0.5 * 0.5,
                tile_width: 100,
                world_half_extents: 250.0,
                world_bottom_bound: -100.0,
                max_traversable_slope_radians: (10.0_f32 - 0.1).to_radians(),
                walkable_height: (npc::HEIGHT * npc::SCALE * npc::RADIUS * npc::SCALE * 0.5 * 0.5)
                    .ceil() as u16,
                walkable_radius: 2,
                step_height: 1,
                min_region_area: 100,
                merge_region_area: 500,
                max_contour_simplification_error: 1.3,
                max_edge_length: 80,
            })
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(query_mesh));
    }
}

#[derive(Debug, Component, Clone, PartialEq, Default, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Follower;

fn query_mesh(
    time: Res<Time>,
    mut with_follower: Query<
        (&GlobalTransform, &mut CharacterVelocity),
        (With<Follower>, Without<Player>),
    >,
    with_player: Query<&GlobalTransform, (With<Player>, Without<Follower>)>,
    nav_mesh_settings: Res<NavMeshSettings>,
    nav_mesh: Res<NavMesh>,
) {
    let dt = time.delta_seconds();
    // Get the underlying nav_mesh.
    if let Ok(nav_mesh) = nav_mesh.get().read() {
        for (follower_transform, mut character_velocity) in &mut with_follower {
            for player_transform in &with_player {
                let start_pos = follower_transform.translation();
                let end_pos = player_transform.translation()
                    + Vec3::Y * (npc::HEIGHT * npc::SCALE - player::HEIGHT * player::SCALE);
                let end_pos = if (end_pos.y - start_pos.y).abs() > 0.1 {
                    end_pos
                } else {
                    Vec3 {
                        y: start_pos.y,
                        ..end_pos
                    }
                };
                info!("start: {}", start_pos);
                info!("end: {}", end_pos);

                // Run pathfinding to get a polygon path.
                match find_path(&nav_mesh, &nav_mesh_settings, start_pos, end_pos, Some(2.)) {
                    Ok(path) => {
                        // Convert polygon path to a path of Vec3s.
                        match perform_string_pulling_on_path(&nav_mesh, start_pos, end_pos, &path) {
                            Ok(string_path) => {
                                info!("path: {:?}", string_path);
                                let next_direction = string_path
                                    .into_iter()
                                    .filter_map(|point| (point - start_pos).try_normalize())
                                    .next()
                                    .unwrap();
                                let speed = 5.0;
                                let velocity = next_direction * speed * dt;
                                info!("velocity: {:?}", velocity);
                                character_velocity.0 += velocity;
                            }
                            Err(error) => error!("Error with string path: {:?}", error),
                        };
                    }
                    Err(error) => error!("Error with pathfinding: {:?}", error),
                }
            }
        }
    }
}
