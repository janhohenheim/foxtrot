use crate::player::{CharacterVelocity, Player};
use crate::spawning::objects::{npc, player};
use crate::GameState;
use bevy::prelude::*;
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
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(query_mesh), //.with_system(_draw_nav_mesh),
            );
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
    nav_mesh_settings: Res<NavMeshSettings>,
    nav_mesh: Res<NavMesh>,
    mut lines: ResMut<DebugLines>,
) {
    let dt = time.delta_seconds();
    if let Ok(nav_mesh) = nav_mesh.get().read() {
        for (follower_transform, mut character_velocity) in &mut with_follower {
            for player_transform in &with_player {
                let start_pos =
                    follower_transform.translation() - Vec3::Y * npc::HEIGHT * npc::SCALE;
                let end_pos =
                    player_transform.translation() - Vec3::Y * player::HEIGHT * player::SCALE;
                let end_pos = if (end_pos.y - start_pos.y).abs() > 0.1 {
                    end_pos
                } else {
                    Vec3 {
                        y: start_pos.y,
                        ..end_pos
                    }
                };
                if (end_pos - start_pos).length_squared() < 0.75 {
                    continue;
                }

                // Run pathfinding to get a polygon path.
                match find_path(&nav_mesh, &nav_mesh_settings, start_pos, end_pos, Some(5.)) {
                    Ok(path) => {
                        // Convert polygon path to a path of Vec3s.
                        match perform_string_pulling_on_path(&nav_mesh, start_pos, end_pos, &path) {
                            Ok(string_path) => {
                                let path = string_path.into_iter().chain(iter::once(end_pos));
                                for (a, b) in path.clone().zip(path.clone().skip(1)) {
                                    lines.line_colored(a, b, 0., Color::RED);
                                }
                                let next_direction = path
                                    .map(|point| point - start_pos)
                                    .map(|dir| {
                                        if dir.y.abs() > 0.25 {
                                            dir
                                        } else {
                                            Vec3 { y: 0., ..dir }
                                        }
                                    })
                                    .filter(|dir| dir.length_squared() > 0.5)
                                    .filter_map(|dir| dir.try_normalize())
                                    .next();
                                let next_direction = match next_direction {
                                    None => {
                                        warn!("No good direction found");
                                        continue;
                                    }
                                    Some(dir) => dir,
                                };
                                let speed = 5.0;
                                let velocity = next_direction * speed * dt;
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

fn _draw_nav_mesh(nav_mesh: Res<NavMesh>, mut lines: ResMut<DebugLines>) {
    // Probably want to add in a trigger key here to make it not always draw.

    if let Ok(nav_mesh) = nav_mesh.get().read() {
        for (_, tile) in nav_mesh.get_tiles().iter() {
            // Draw polygons.
            for poly in tile.polygons.iter() {
                let indices = &poly.indices;
                for i in 0..indices.len() {
                    let a = tile.vertices[indices[i] as usize];
                    let b = tile.vertices[indices[(i + 1) % indices.len()] as usize];

                    lines.line(a, b, 0.0);
                }
            }

            // Draw vertex points.
            for vertex in tile.vertices.iter() {
                lines.line(*vertex, *vertex + Vec3::Y, 0.0);
            }
        }
    }
}
