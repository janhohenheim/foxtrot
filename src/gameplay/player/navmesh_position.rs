use bevy::prelude::*;
use bevy_landmass::{Archipelago3d, FromAgentRadius, PointSampleDistance3d};
#[cfg(feature = "hot_patch")]
use bevy_simple_subsecond_system::hot;

use crate::PrePhysicsAppSystems;

use super::PLAYER_RADIUS;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        RunFixedMainLoop,
        update_last_valid_player_navmesh_position
            .in_set(PrePhysicsAppSystems::UpdateNavmeshPositions),
    );
}

#[derive(Component, Reflect, Default, Deref, DerefMut)]
#[reflect(Component, Default)]
pub(crate) struct LastValidPlayerNavmeshPosition(pub(crate) Option<Vec3>);

#[cfg_attr(feature = "hot_patch", hot)]
fn update_last_valid_player_navmesh_position(
    player_character: Single<(&GlobalTransform, &mut LastValidPlayerNavmeshPosition)>,
    archipelago: Single<&Archipelago3d>,
) {
    let (transform, mut last_valid_player_navmesh_position) = player_character.into_inner();
    let sampled_point = archipelago.sample_point(
        transform.translation(),
        &PointSampleDistance3d::from_agent_radius(PLAYER_RADIUS * 2.0),
    );
    if let Ok(sampled_point) = sampled_point {
        last_valid_player_navmesh_position.0 = Some(sampled_point.point());
    }
}
