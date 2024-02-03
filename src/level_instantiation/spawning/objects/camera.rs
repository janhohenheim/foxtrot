use crate::player_control::actions::create_camera_action_input_manager_bundle;
use crate::player_control::camera::IngameCamera;
use bevy::prelude::*;
use bevy_atmosphere::prelude::*;
use bevy_dolly::prelude::*;
#[cfg(feature = "dev")]
use bevy_editor_pls::default_windows::cameras::EditorCamera;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub(crate) struct IngameCameraMarker;

pub(crate) fn spawn(camera: Query<Entity, Added<IngameCameraMarker>>, mut commands: Commands) {
    for entity in camera.iter() {
        commands.entity(entity).insert((
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
            #[cfg(feature = "dev")]
            EditorCamera,
        ));
    }
}
