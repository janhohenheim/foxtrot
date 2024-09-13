use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::player::Player;

use super::PlayerCamera;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<FirstPersonCamera>();
    app.add_plugins((InputManagerPlugin::<CameraAction>::default(),));
    app.add_systems(Update, (rotate_camera, follow_player).chain());
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

#[derive(Debug, Clone, Copy, PartialEq, Component, Reflect, Default)]
#[reflect(Component)]
pub struct FirstPersonCamera {
    pub follow: Transform,
    pub offset: Transform,
    pub look_at: Option<Vec3>,
}

impl FirstPersonCamera {
    pub fn transform(self) -> Transform {
        self.follow * self.offset
    }
}

pub fn first_person_camera_bundle() -> impl Bundle {
    (
        Name::new("Camera"),
        Camera3dBundle::default(),
        FirstPersonCamera::default(),
        IsDefaultUiCamera,
        PlayerCamera::default(),
        InputManagerBundle::with_map(CameraAction::default_input_map()),
    )
}

fn follow_player(
    mut q_camera: Query<&mut Transform, With<FirstPersonCamera>>,
    q_player: Query<&Transform, (With<Player>, Without<FirstPersonCamera>)>,
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
    mut character_query: Query<(&mut Transform, &ActionState<CameraAction>, &PlayerCamera)>,
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
