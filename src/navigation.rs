use crate::player::{CharacterVelocity, Player};
use crate::GameState;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_pathmesh::PathMesh;
use bevy_pathmesh::PathmeshPlugin;
use bevy_prototype_debug_lines::DebugLines;
use serde::{Deserialize, Serialize};

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
                let from = follower_transform.translation().xz();
                let to = player_transform.translation().xz();
                let path = path_mesh.path(from, to);
                if let Some(path) = path {
                    info!("{:?}", path);
                }
            }
        }
    }
}
