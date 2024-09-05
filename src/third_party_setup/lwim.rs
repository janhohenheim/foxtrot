use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        InputManagerPlugin::<PlayerAction>::default(),
        InputManagerPlugin::<CameraAction>::default(),
    ));
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Actionlike, Reflect, Default)]
pub(crate) enum PlayerAction {
    #[default]
    #[actionlike(DualAxis)]
    Move,
    Sprint,
    Jump,
    Interact,
}

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
#[actionlike(DualAxis)]
pub enum CameraAction {
    RotateCamera,
}

impl PlayerAction {
    pub fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        // Default gamepad input bindings
        input_map.insert(PlayerAction::PedalLeft, GamepadButtonType::LeftTrigger);
        input_map.insert(PlayerAction::PedalRight, GamepadButtonType::RightTrigger);

        // Default keyboard input bindings
        input_map.insert(PlayerAction::PedalLeft, KeyCode::KeyA);
        input_map.insert(PlayerAction::PedalRight, KeyCode::KeyD);

        input_map
    }
}

impl CameraAction {
    pub fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        // Default gamepad input bindings
        input_map.insert(CameraAction::RotateCamera, DualAxis::left_stick());

        // Default keyboard input bindings
        input_map.insert(CameraAction::RotateCamera, DualAxis::mouse_motion());

        input_map
    }
}
