use std::f32::consts::FRAC_PI_2;

use crate::{dialog::conditions::dialog_running, player::Player};
use bevy::{app::RunFixedMainLoop, prelude::*, time::run_fixed_main_schedule};
use leafwing_input_manager::prelude::*;

use super::{PlayerCamera, PlayerCameraConfig};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(InputManagerPlugin::<CameraAction>::default());
    app.add_systems(
        RunFixedMainLoop,
        rotate_camera
            .run_if(not(dialog_running))
            .before(run_fixed_main_schedule),
    );
    app.add_systems(Update, follow_player);
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
#[actionlike(DualAxis)]
pub enum CameraAction {
    RotateCamera,
}

impl CameraAction {
    pub fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        // Default gamepad input bindings
        input_map.insert_dual_axis(CameraAction::RotateCamera, GamepadStick::LEFT);

        // Default kbm input bindings
        input_map.insert_dual_axis(CameraAction::RotateCamera, MouseMove::default());

        input_map
    }
}

fn follow_player(
    mut q_camera: Query<&mut Transform, With<PlayerCamera>>,
    q_player: Query<&Transform, (With<Player>, Without<PlayerCamera>)>,
) {
    // Use `Transform` instead of `Position`` because we want the camera to move
    // smoothly, so we use the interpolated transform of the player.
    let Ok(player_transform) = q_player.get_single() else {
        return;
    };
    let Ok(mut camera_transform) = q_camera.get_single_mut() else {
        return;
    };
    let height_offset = 0.5;
    camera_transform.translation =
        player_transform.translation + player_transform.up() * height_offset;
}

fn rotate_camera(
    mut character_query: Query<(
        &mut Transform,
        &ActionState<CameraAction>,
        &PlayerCameraConfig,
    )>,
) {
    for (mut transform, action_state, config) in &mut character_query {
        let delta = action_state.axis_pair(&CameraAction::RotateCamera);
        let delta_yaw = -delta.x * config.sensitivity.x;
        let delta_pitch = -delta.y * config.sensitivity.y;

        let (yaw, pitch, roll) = transform.rotation.to_euler(EulerRot::YXZ);
        let yaw = yaw + delta_yaw;

        const PITCH_LIMIT: f32 = FRAC_PI_2 - 0.01;
        let pitch = (pitch + delta_pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);

        transform.rotation = Quat::from_euler(EulerRot::YXZ, yaw, pitch, roll);
    }
}
