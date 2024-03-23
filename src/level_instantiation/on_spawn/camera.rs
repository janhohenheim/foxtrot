use crate::{
    player_control::{actions::create_camera_action_input_manager_bundle, camera::IngameCamera},
    GameState,
};
use bevy::prelude::*;
use bevy_atmosphere::prelude::*;
use bevy_dolly::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
struct IngameCameraMarker;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<IngameCameraMarker>()
        .add_systems(Update, spawn.run_if(in_state(GameState::Playing)));
}

fn spawn(camera: Query<Entity, Added<IngameCameraMarker>>, mut commands: Commands) {
    for entity in camera.iter() {
        commands
            .entity(entity)
            .insert((
                Camera3dBundle::default(),
                IngameCamera::default(),
                AtmosphereCamera::default(),
                Rig::builder()
                    .with(Position::new(default()))
                    .with(YawPitch::new())
                    .with(Smooth::new_position_rotation(default(), default()))
                    .with(Arm::new(default()))
                    .with(LookAt::new(default()).tracking_predictive(true))
                    .build(),
                create_camera_action_input_manager_bundle(),
            ))
            .remove::<IngameCameraMarker>();
    }
}
