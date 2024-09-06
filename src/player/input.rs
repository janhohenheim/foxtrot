use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::character::action::CharacterAction;

pub(super) fn plugin(_app: &mut App) {}

impl CharacterAction {
    pub fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        // Default gamepad input bindings
        input_map.insert_dual_axis(Self::Move, GamepadStick::LEFT);
        input_map.insert(Self::Jump, GamepadButtonType::South);
        input_map.insert(Self::Interact, GamepadButtonType::West);
        input_map.insert(Self::Sprint, GamepadButtonType::LeftTrigger);

        // Default kbm input bindings
        input_map.insert_dual_axis(Self::Move, KeyboardVirtualDPad::WASD);
        input_map.insert(Self::Jump, KeyCode::Space);
        input_map.insert(Self::Interact, MouseButton::Left);
        input_map.insert(Self::Sprint, KeyCode::ShiftLeft);

        input_map
    }
}
