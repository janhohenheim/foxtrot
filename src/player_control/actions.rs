use crate::util::criteria::is_frozen;
use bevy::prelude::*;
use leafwing_input_manager::axislike::DualAxisData;
use leafwing_input_manager::plugin::InputManagerSystem;
use leafwing_input_manager::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Resource, Default, Reflect, Serialize, Deserialize)]
#[reflect(Resource, Serialize, Deserialize)]
pub(crate) struct ActionsFrozen {
    freeze_count: usize,
}
impl ActionsFrozen {
    pub(crate) fn freeze(&mut self) {
        self.freeze_count += 1;
    }
    pub(crate) fn unfreeze(&mut self) {
        self.freeze_count -= 1;
    }
    pub(crate) fn is_frozen(&self) -> bool {
        self.freeze_count > 0
    }
}

/// Configures [`Actions`], the resource that holds all player input.
/// Add new input in [`set_actions`] and in [`game_control::generate_bindings!`](game_control).

pub(crate) fn actions_plugin(app: &mut App) {
    app.register_type::<PlayerAction>()
        .register_type::<CameraAction>()
        .register_type::<UiAction>()
        .register_type::<ActionsFrozen>()
        .init_resource::<ActionsFrozen>()
        .add_plugin(InputManagerPlugin::<PlayerAction>::default())
        .add_plugin(InputManagerPlugin::<CameraAction>::default())
        .add_plugin(InputManagerPlugin::<UiAction>::default())
        .add_system(
            remove_actions_when_frozen
                .run_if(is_frozen)
                .after(InputManagerSystem::ManualControl)
                .in_base_set(CoreSet::PreUpdate),
        );
}

#[derive(Debug, Clone, Copy, Actionlike, Reflect, FromReflect, Default)]
pub(crate) enum PlayerAction {
    #[default]
    Move,
    Sprint,
    Jump,
    Interact,
    SpeedUpDialog,
    NumberedChoice1,
    NumberedChoice2,
    NumberedChoice3,
    NumberedChoice4,
    NumberedChoice5,
    NumberedChoice6,
    NumberedChoice7,
    NumberedChoice8,
    NumberedChoice9,
    NumberedChoice0,
}

impl PlayerAction {
    pub(crate) fn numbered_choice(index: u8) -> Self {
        match index {
            0 => PlayerAction::NumberedChoice0,
            1 => PlayerAction::NumberedChoice1,
            2 => PlayerAction::NumberedChoice2,
            3 => PlayerAction::NumberedChoice3,
            4 => PlayerAction::NumberedChoice4,
            5 => PlayerAction::NumberedChoice5,
            6 => PlayerAction::NumberedChoice6,
            7 => PlayerAction::NumberedChoice7,
            8 => PlayerAction::NumberedChoice8,
            9 => PlayerAction::NumberedChoice9,
            _ => panic!(
                "Numbered choice index out of range: got {}, expected 0-9",
                index
            ),
        }
    }
}

#[derive(Debug, Clone, Actionlike, Reflect, FromReflect, Default)]
pub(crate) enum CameraAction {
    #[default]
    Orbit,
    Zoom,
}

#[derive(Debug, Clone, Actionlike, Reflect, FromReflect, Default)]
pub(crate) enum UiAction {
    #[default]
    TogglePause,
}

pub(crate) fn create_player_action_input_manager_bundle() -> InputManagerBundle<PlayerAction> {
    InputManagerBundle {
        input_map: InputMap::new([
            (QwertyScanCode::Space, PlayerAction::Jump),
            (QwertyScanCode::LShift, PlayerAction::Sprint),
            (QwertyScanCode::E, PlayerAction::Interact),
            (QwertyScanCode::Space, PlayerAction::SpeedUpDialog),
            (QwertyScanCode::Key1, PlayerAction::NumberedChoice1),
            (QwertyScanCode::Key2, PlayerAction::NumberedChoice2),
            (QwertyScanCode::Key3, PlayerAction::NumberedChoice3),
            (QwertyScanCode::Key4, PlayerAction::NumberedChoice4),
            (QwertyScanCode::Key5, PlayerAction::NumberedChoice5),
            (QwertyScanCode::Key6, PlayerAction::NumberedChoice6),
            (QwertyScanCode::Key7, PlayerAction::NumberedChoice7),
            (QwertyScanCode::Key8, PlayerAction::NumberedChoice8),
            (QwertyScanCode::Key9, PlayerAction::NumberedChoice9),
            (QwertyScanCode::Key0, PlayerAction::NumberedChoice0),
        ])
        .insert(VirtualDPad::wasd(), PlayerAction::Move)
        .build(),
        ..default()
    }
}

pub(crate) fn create_camera_action_input_manager_bundle() -> InputManagerBundle<CameraAction> {
    InputManagerBundle {
        input_map: InputMap::default()
            .insert(DualAxis::mouse_motion(), CameraAction::Orbit)
            .insert(SingleAxis::mouse_wheel_y(), CameraAction::Zoom)
            .build(),
        ..default()
    }
}

pub(crate) fn create_ui_action_input_manager_bundle() -> InputManagerBundle<UiAction> {
    InputManagerBundle {
        input_map: InputMap::new([(QwertyScanCode::Escape, UiAction::TogglePause)]),
        ..default()
    }
}

pub(crate) fn remove_actions_when_frozen(
    mut player_actions_query: Query<&mut ActionState<PlayerAction>>,
    mut camera_actions_query: Query<&mut ActionState<CameraAction>>,
) {
    for mut player_actions in player_actions_query.iter_mut() {
        player_actions.action_data_mut(PlayerAction::Move).axis_pair = Some(default());
        player_actions.release(PlayerAction::Jump);
        player_actions.release(PlayerAction::Interact);
        player_actions.release(PlayerAction::Sprint);
    }
    for mut camera_actions in camera_actions_query.iter_mut() {
        camera_actions
            .action_data_mut(CameraAction::Orbit)
            .axis_pair = Some(default());
        camera_actions.action_data_mut(CameraAction::Zoom).value = default();
    }
}

pub(crate) trait DualAxisDataExt {
    fn max_normalized(self) -> Option<Vec2>;
}

impl DualAxisDataExt for DualAxisData {
    fn max_normalized(self) -> Option<Vec2> {
        let vector = self.xy();
        let len_squared = vector.length_squared();
        if len_squared > 1.0 {
            Some(vector.normalize())
        } else if len_squared < 1e-5 {
            None
        } else {
            Some(vector)
        }
    }
}
