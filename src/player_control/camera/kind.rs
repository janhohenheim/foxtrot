use crate::file_system_interaction::config::GameConfig;
use crate::player_control::actions::CameraAction;
use crate::player_control::camera::{IngameCamera, IngameCameraKind};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub fn update_kind(
    mut camera_query: Query<(&mut IngameCamera, &ActionState<CameraAction>)>,
    config: Res<GameConfig>,
) {
    for (mut camera, actions) in camera_query.iter_mut() {
        let zoom = actions.clamped_value(CameraAction::Zoom);
        let zoomed_out = zoom < -1e-5;
        let zoomed_in = zoom > 1e-5;
        let new_kind = match camera.kind {
            IngameCameraKind::FirstPerson if zoomed_out => Some(IngameCameraKind::ThirdPerson),
            IngameCameraKind::ThirdPerson
                if camera.desired_distance < config.camera.third_person.min_distance + 1e-5
                    && zoomed_in =>
            {
                Some(IngameCameraKind::FirstPerson)
            }
            IngameCameraKind::ThirdPerson
                if camera.desired_distance > config.camera.third_person.max_distance - 1e-5
                    && zoomed_out =>
            {
                Some(IngameCameraKind::FixedAngle)
            }
            IngameCameraKind::FixedAngle
                if camera.desired_distance < config.camera.fixed_angle.min_distance + 1e-5
                    && zoomed_in =>
            {
                Some(IngameCameraKind::ThirdPerson)
            }
            _ => None,
        };
        if let Some(new_kind) = new_kind {
            camera.kind = new_kind;
        }
    }
}
