use crate::player_control::actions::create_camera_action_input_manager_bundle;
use crate::player_control::camera::IngameCamera;
use bevy::prelude::*;
use bevy_dolly::prelude::*;
#[cfg(feature = "dev")]
use bevy_editor_pls::default_windows::cameras::EditorCamera;

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Reflect, Component)]
#[reflect(Component)]
pub(crate) struct IngameCameraMarker;

pub(crate) fn spawn(camera: Query<Entity, Added<IngameCameraMarker>>, mut commands: Commands) {
    for entity in camera.iter() {
        commands.entity(entity).insert((
            IngameCamera::default(),
            Rig::builder()
                .with(Position::new(default()))
                .with(YawPitch::new())
                .with(Smooth::new_position_rotation(default(), default()))
                .with(Arm::new(default()))
                .with(LookAt::new(default()).tracking_predictive(true))
                .build(),
            create_camera_action_input_manager_bundle(),
            Name::new("Main Camera"),
            #[cfg(feature = "dev")]
            EditorCamera,
        ));
    }
}
