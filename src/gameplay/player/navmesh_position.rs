use std::time::Duration;

use bevy::prelude::*;
use bevy_landmass::{Archipelago3d, FromAgentRadius, PointSampleDistance3d};

use super::PLAYER_RADIUS;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(FixedUpdate, update_last_valid_player_navmesh_position);
}

#[derive(Component, Reflect, Default, Deref, DerefMut)]
#[reflect(Component, Default)]
pub(crate) struct LastValidPlayerNavmeshPosition(pub(crate) Option<Vec3>);

fn update_last_valid_player_navmesh_position(
    player_character: Single<(&GlobalTransform, &mut LastValidPlayerNavmeshPosition)>,
    archipelago: Single<&Archipelago3d>,
    time: Res<Time>,
    mut timer: Local<Option<Timer>>,
) {
    let timer = timer.get_or_insert(Timer::new(Duration::from_millis(500), TimerMode::Once));
    timer.tick(time.delta());
    if !timer.is_finished() {
        return;
    }
    timer.reset();
    let (transform, mut last_valid_player_navmesh_position) = player_character.into_inner();
    let sampled_point = archipelago.sample_point(
        transform.translation(),
        &PointSampleDistance3d::from_agent_radius(PLAYER_RADIUS * 2.0),
    );
    if let Ok(sampled_point) = sampled_point {
        last_valid_player_navmesh_position.0 = Some(sampled_point.point());
    }
}
