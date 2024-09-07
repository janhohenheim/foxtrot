use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::player::Player;

use super::PlayerCamera;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<FirstPersonCamera>();
    app.add_plugins((InputManagerPlugin::<CameraAction>::default(),));
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
        PlayerCamera,
        InputManagerBundle::with_map(CameraAction::default_input_map()),
    )
}

fn follow_player(
    time: Res<Time>,
    mut q_camera: Query<&mut Transform, With<FirstPersonCamera>>,
    q_player: Query<&Transform, (With<Player>, Without<FirstPersonCamera>)>,
) {
    let dt = time.delta_seconds();
    let Ok(player_transform) = q_player.get_single() else {
        return;
    };
    let Ok(mut camera_transform) = q_camera.get_single_mut() else {
        return;
    };
    let decay_rate = f32::ln(5.0);
    let origin = &mut camera_transform.translation;
    let target = player_transform.translation;
    *origin = origin.lerp(target, 1.0 - f32::exp(-decay_rate * dt));
}
