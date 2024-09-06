use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<PlayerCamera>();
    app.add_plugins((InputManagerPlugin::<CameraAction>::default(),));
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
pub struct PlayerCamera {
    pub follow: Transform,
    pub offset: Transform,
    pub look_at: Option<Vec3>,
}

impl PlayerCamera {
    pub fn transform(self) -> Transform {
        self.follow * self.offset
    }
}

pub fn spawn_player_camera(world: &mut World) {
    world.spawn((
        Name::new("Camera"),
        Camera3dBundle::default(),
        PlayerCamera::default(),
        IsDefaultUiCamera,
        InputManagerBundle::with_map(CameraAction::default_input_map()),
    ));
}
