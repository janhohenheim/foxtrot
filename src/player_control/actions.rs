use crate::player_control::actions::game_control::{get_movement, GameControl};
use crate::util::trait_extension::{F32Ext, Vec2Ext};
use crate::GameState;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

mod game_control;

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
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(set_actions));
    }
}

#[derive(
    Debug, Clone, PartialEq, Reflect, FromReflect, Default, Resource, Serialize, Deserialize,
)]
#[reflect(Resource, Serialize, Deserialize)]
pub struct Actions {
    pub player: PlayerActions,
    pub camera: CameraActions,
    pub ui: UiActions,
}

#[derive(Debug, Clone, PartialEq, Reflect, FromReflect, Default, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub struct PlayerActions {
    pub movement: Option<Vec2>,
    pub interact: bool,
    pub jump: bool,
    pub sprint: bool,
}

#[derive(Debug, Clone, PartialEq, Reflect, FromReflect, Default, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub struct CameraActions {
    pub movement: Option<Vec2>,
    pub zoom: Option<f32>,
}

#[derive(Debug, Clone, PartialEq, Reflect, FromReflect, Default, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub struct UiActions {
    pub toggle_editor: bool,
    pub numbered_choice: [bool; 10],
}

pub fn set_actions(
    mut actions: ResMut<Actions>,
    keyboard_input: Res<Input<ScanCode>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut mouse_wheel: EventReader<MouseWheel>,
    actions_frozen: Res<ActionsFrozen>,
) {
    *actions = default();
    actions.ui.toggle_editor = GameControl::ToggleEditor.just_pressed(&keyboard_input);
    for i in 0..=9 {
        actions.ui.numbered_choice[i] =
            GameControl::NumberedChoice(i as u16).just_pressed(&keyboard_input);
    }
    if actions_frozen.is_frozen() {
        return;
    }

    let player_movement = Vec2::new(
        get_movement(GameControl::Right, &keyboard_input)
            - get_movement(GameControl::Left, &keyboard_input),
        get_movement(GameControl::Up, &keyboard_input)
            - get_movement(GameControl::Down, &keyboard_input),
    );

    actions.player.movement = (player_movement != Vec2::ZERO).then(|| player_movement.normalize());
    actions.player.jump = get_movement(GameControl::Jump, &keyboard_input) > 0.5;
    actions.player.sprint = get_movement(GameControl::Sprint, &keyboard_input) > 0.5;
    actions.player.interact = GameControl::Interact.just_pressed(&keyboard_input);

    let mut camera_movement = Vec2::ZERO;
    for event in mouse_motion.iter() {
        camera_movement += event.delta;
    }
    if !camera_movement.is_approx_zero() {
        actions.camera.movement = Some(camera_movement);
    }

    let mut zoom = 0.0;
    for event in mouse_wheel.iter() {
        zoom += event.y;
    }
    if !zoom.is_approx_zero() {
        actions.camera.zoom = Some(zoom);
    }
}
