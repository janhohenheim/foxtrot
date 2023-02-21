use crate::player_control::actions::game_control::{get_movement, Action};
use crate::util::trait_extension::{F32Ext, Vec2Ext};
use crate::GameState;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
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
        app.register_type::<Actions>()
            .register_type::<PlayerActions>()
            .register_type::<CameraActions>()
            .register_type::<UiActions>()
            .register_type::<ActionsFrozen>()
            .init_resource::<Actions>()
            .init_resource::<ActionsFrozen>()
            .add_plugin(InputManagerPlugin::<PlayerAction>::default())
            .add_plugin(InputManagerPlugin::<CameraAction>::default())
            .add_plugin(InputManagerPlugin::<UiAction>::default())
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(set_actions));
    }
}

#[derive(Debug, Clone, Actionlike)]
pub enum PlayerAction {
    Move,
    Sprint,
    Jump,
    Interact,
    SpeedUpDialog,
    NumberedChoice(u16),
}

#[derive(Debug, Clone, Actionlike)]
pub enum CameraAction {
    Pan,
    Zoom,
}

#[derive(Debug, Clone, Actionlike)]
pub enum UiAction {
    #[cfg(feature = "dev")]
    ToggleEditor,
    TogglePause,
}

pub fn create_player_action_input_manager_bundle() -> InputManagerBundle<PlayerAction> {
    InputManagerBundle {
        input_map: InputMap::new([
            (QwertyScanCode::Space, PlayerAction::Jump),
            (
                VirtualDPad {
                    up: QwertyScanCode::W,
                    down: QwertyScanCode::S,
                    left: QwertyScanCode::A,
                    right: QwertyScanCode::D,
                },
                PlayerAction::Move,
            ),
            (QwertyScanCode::LShift, PlayerAction::Sprint),
            (QwertyScanCode::E, PlayerAction::Interact),
            (QwertyScanCode::Space, UiAction::SpeedUpDialog),
            (QwertyScanCode::Key1, UiAction::NumberedChoice(1)),
            (QwertyScanCode::Key2, UiAction::NumberedChoice(2)),
            (QwertyScanCode::Key3, UiAction::NumberedChoice(3)),
            (QwertyScanCode::Key4, UiAction::NumberedChoice(4)),
            (QwertyScanCode::Key5, UiAction::NumberedChoice(5)),
            (QwertyScanCode::Key6, UiAction::NumberedChoice(6)),
            (QwertyScanCode::Key7, UiAction::NumberedChoice(7)),
            (QwertyScanCode::Key8, UiAction::NumberedChoice(8)),
            (QwertyScanCode::Key9, UiAction::NumberedChoice(9)),
            (QwertyScanCode::Key0, UiAction::NumberedChoice(0)),
        ]),
        ..default()
    }
}

pub fn create_camera_action_input_manager_bundle() -> InputManagerBundle<CameraAction> {
    InputManagerBundle {
        input_map: InputMap::default()
            .insert(DualAxis::mouse_motion(), CameraAction::Pan)
            .insert(DualAxis::mouse_wheel(), CameraAction::Zoom),
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

pub fn set_actions(
    mut actions: ResMut<Actions>,
    keyboard_input: Res<Input<ScanCode>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut mouse_wheel: EventReader<MouseWheel>,
    actions_frozen: Res<ActionsFrozen>,
) {
    #[cfg(feature = "tracing")]
    let _span = info_span!("set_actions").entered();
    *actions = default();
    #[cfg(feature = "dev")]
    {
        actions.ui.toggle_editor = Action::ToggleEditor.just_pressed(&keyboard_input);
    }
    actions.ui.toggle_pause = Action::TogglePause.just_pressed(&keyboard_input);
    actions.ui.speed_up_dialog = Action::SpeedUpDialog.pressed(&keyboard_input);
    for i in 0..=9 {
        actions.ui.numbered_choice[i] =
            Action::NumberedChoice(i as u16).just_pressed(&keyboard_input);
    }
    if actions_frozen.is_frozen() {
        return;
    }

    let player_movement = Vec2::new(
        get_movement(Action::Right, &keyboard_input) - get_movement(Action::Left, &keyboard_input),
        get_movement(Action::Up, &keyboard_input) - get_movement(Action::Down, &keyboard_input),
    );

    actions.player.movement =
        (!player_movement.is_approx_zero()).then(|| player_movement.normalize());
    actions.player.jump = get_movement(Action::Jump, &keyboard_input) > 0.5;
    actions.player.sprint = get_movement(Action::Sprint, &keyboard_input) > 0.5;
    actions.player.interact = Action::Interact.just_pressed(&keyboard_input);

    let mut camera_movement = Vec2::ZERO;
    for event in mouse_motion.iter() {
        camera_movement += event.delta;
    }
    if !camera_movement.is_approx_zero() {
        actions.camera.movement = Some(camera_movement);
    }

    let mut zoom = 0.0;
    for event in mouse_wheel.iter() {
        zoom += event.y.signum();
    }
    if !zoom.is_approx_zero() {
        actions.camera.zoom = Some(zoom);
    }
}
