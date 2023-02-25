use bevy::prelude::*;
use leafwing_input_manager::axislike::DualAxisData;
use leafwing_input_manager::plugin::InputManagerSystem;
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
            .add_plugin(InputManagerPlugin::<UiAction>::default())
            .add_system_to_stage(
                CoreStage::PreUpdate,
                remove_actions_when_frozen.after(InputManagerSystem::ManualControl),
            );
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
        .insert(VirtualDPad::wasd(), PlayerAction::Move)
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
        input_map: InputMap::new([(QwertyScanCode::Escape, UiAction::TogglePause)]),
        ..default()
    }
}

pub fn remove_actions_when_frozen(
    actions_frozen: Res<ActionsFrozen>,
    mut player_actions_query: Query<&mut ActionState<PlayerAction>>,
    mut camera_actions_query: Query<&mut ActionState<CameraAction>>,
) {
    if actions_frozen.is_frozen() {
        for mut player_actions in player_actions_query.iter_mut() {
            player_actions.action_data_mut(PlayerAction::Move).axis_pair = Some(default());
            player_actions.release(PlayerAction::Jump);
            player_actions.release(PlayerAction::Interact);
            player_actions.release(PlayerAction::Sprint);
        }
        for mut camera_actions in camera_actions_query.iter_mut() {
            camera_actions.action_data_mut(CameraAction::Pan).axis_pair = Some(default());
            camera_actions.action_data_mut(CameraAction::Zoom).value = default();
        }
    }
}

pub trait DualAxisDataExt {
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
