use crate::player::{CharacterVelocity, Player};
use crate::GameState;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_pathmesh::PathMesh;
use bevy_pathmesh::PathmeshPlugin;
use bevy_prototype_debug_lines::DebugLines;
use serde::{Deserialize, Serialize};
use std::iter;

pub struct NavigationPlugin;

impl Plugin for NavigationPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PathmeshPlugin)
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
    nav_meshes: Query<&Handle<PathMesh>>,
) {
    let dt = time.delta_seconds();
    for path_mesh_handle in nav_meshes.iter() {
        for (follower_transform, mut character_velocity) in &mut with_follower {
            for player_transform in &with_player {
                let path_mesh = path_meshes.get(path_mesh_handle).unwrap();
                if let Some(from) = get_closest_vertex(&follower_transform, &path_mesh)
                    && let Some(to) = get_closest_vertex(&player_transform, &path_mesh)
                    && let Some(path) = path_mesh.path(from, to) {
                    move_along_path(follower_transform.translation().xz(), player_transform.translation().xz(), path.path, &mut lines, &mut character_velocity, dt);
                }
            }
        }
    }
}

fn move_along_path(
    from: Vec2,
    to: Vec2,
    path: Vec<Vec2>,
    lines: &mut ResMut<DebugLines>,
    character_velocity: &mut Mut<CharacterVelocity>,
    dt: f32,
) {
    let from = Vec3::new(from.x, 0., from.y);
    let to = Vec3::new(to.x, 0., to.y);
    let path: Vec<_> = path
        .iter()
        .map(|vec2| Vec3::new(vec2.x, 0., vec2.y))
        .collect();

    let line_path = iter::once(&from).chain(path.iter()).chain(iter::once(&to));
    for (a, b) in line_path.clone().zip(line_path.skip(1)) {
        let visibility_offset = Vec3::Y * 0.5;
        lines.line_colored(
            *a + visibility_offset,
            *b + visibility_offset,
            0.,
            Color::RED,
        );
    }
    let next_direction = path
        .into_iter()
        .map(|point| point - from)
        .filter_map(|dir| dir.try_normalize())
        .next()
        .unwrap();
    let speed = 5.0;
    let velocity = next_direction * speed * dt;
    character_velocity.0 += velocity;
}

fn get_closest_vertex(transform: &GlobalTransform, path_mesh: &PathMesh) -> Option<Vec2> {
    let coords = transform.translation().xz();
    let distance_per_try = 0.09;
    let tries = 10;
    for x in 0..tries {
        for y in 0..tries {
            let offset = Vec2::new(x as f32, y as f32) * distance_per_try;
            if path_mesh.is_in_mesh(coords + offset) {
                return Some(coords + offset);
            } else if path_mesh.is_in_mesh(coords - offset) {
                return Some(coords - offset);
            }
        }
    }
    None
}
