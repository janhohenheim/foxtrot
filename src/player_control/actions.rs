use bevy::prelude::*;
use leafwing_input_manager::axislike::DualAxisData;
use leafwing_input_manager::prelude::*;
use serde::{Deserialize, Serialize};

/// Configures [`Actions`], the resource that holds all player input.
/// Add new input in [`set_actions`] and in [`game_control::generate_bindings!`](game_control).
pub struct ActionsPlugin;

#[derive(Resource, Default, Reflect, Serialize, Deserialize)]
#[reflect(Resource, Serialize, Deserialize)]
pub struct ActionsFrozen {
    freeze_count: usize,
}
impl ActionsFrozen {
    pub fn freeze(&mut self) {
        self.freeze_count += 1;
    }
    pub fn unfreeze(&mut self) {
        self.freeze_count -= 1;
    }
    pub fn is_frozen(&self) -> bool {
        self.freeze_count > 0
    }
}

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PlayerAction>()
            .register_type::<CameraAction>()
            .register_type::<UiAction>()
            .register_type::<ActionsFrozen>()
            .init_resource::<ActionsFrozen>()
            .add_plugin(InputManagerPlugin::<PlayerAction>::default())
            .add_plugin(InputManagerPlugin::<CameraAction>::default())
            .add_plugin(InputManagerPlugin::<UiAction>::default());
    }
}

#[derive(Debug, Clone, Actionlike, Reflect, FromReflect, Default)]
pub enum PlayerAction {
    #[default]
    Move,
    Sprint,
    Jump,
    Interact,
    SpeedUpDialog,
    NumberedChoice(u16),
}

#[derive(Debug, Clone, Actionlike, Reflect, FromReflect, Default)]
pub enum CameraAction {
    #[default]
    Pan,
    Zoom,
}

#[derive(Debug, Clone, Actionlike, Reflect, FromReflect, Default)]
pub enum UiAction {
    #[cfg(feature = "dev")]
    ToggleEditor,
    #[default]
    TogglePause,
}

pub fn create_player_action_input_manager_bundle() -> InputManagerBundle<PlayerAction> {
    InputManagerBundle {
        input_map: InputMap::new([
            (QwertyScanCode::Space, PlayerAction::Jump),
            (QwertyScanCode::LShift, PlayerAction::Sprint),
            (QwertyScanCode::E, PlayerAction::Interact),
            (QwertyScanCode::Space, PlayerAction::SpeedUpDialog),
            (QwertyScanCode::Key1, PlayerAction::NumberedChoice(1)),
            (QwertyScanCode::Key2, PlayerAction::NumberedChoice(2)),
            (QwertyScanCode::Key3, PlayerAction::NumberedChoice(3)),
            (QwertyScanCode::Key4, PlayerAction::NumberedChoice(4)),
            (QwertyScanCode::Key5, PlayerAction::NumberedChoice(5)),
            (QwertyScanCode::Key6, PlayerAction::NumberedChoice(6)),
            (QwertyScanCode::Key7, PlayerAction::NumberedChoice(7)),
            (QwertyScanCode::Key8, PlayerAction::NumberedChoice(8)),
            (QwertyScanCode::Key9, PlayerAction::NumberedChoice(9)),
            (QwertyScanCode::Key0, PlayerAction::NumberedChoice(0)),
        ])
        .insert(
            VirtualDPad {
                up: QwertyScanCode::W.into(),
                down: QwertyScanCode::S.into(),
                left: QwertyScanCode::A.into(),
                right: QwertyScanCode::D.into(),
            },
            PlayerAction::Move,
        )
        .build(),
        ..default()
    }
}

pub fn create_camera_action_input_manager_bundle() -> InputManagerBundle<CameraAction> {
    InputManagerBundle {
        input_map: InputMap::default()
            .insert(DualAxis::mouse_motion(), CameraAction::Pan)
            .insert(SingleAxis::mouse_wheel_y(), CameraAction::Zoom)
            .build(),
        ..default()
    }
}

pub fn create_ui_action_input_manager_bundle() -> InputManagerBundle<UiAction> {
    InputManagerBundle {
        input_map: InputMap::new([
            #[cfg(feature = "dev")]
            (QwertyScanCode::Q, UiAction::ToggleEditor),
            (QwertyScanCode::Escape, UiAction::TogglePause),
        ]),
        ..default()
    }
}

pub trait DualAxisDataExt {
    fn max_normalized(self) -> Option<Vec2>;
}

impl DualAxisDataExt for DualAxisData {
    fn max_normalized(self) -> Option<Vec2> {
        let vect = self.xy();
        let len_squared = vect.length_squared();
        if len_squared > 1.0 {
            Some(vect.normalize())
        } else if len_squared < 1e-5 {
            None
        } else {
            Some(vect)
        }
    }
}
