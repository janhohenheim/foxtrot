use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

use crate::player_control::actions::game_control::{get_movement, GameControl};
use crate::GameState;

mod game_control;

pub struct ActionsPlugin;

#[derive(Resource, Default)]
pub struct ActionsFrozen;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Actions>()
            .register_type::<PlayerActions>()
            .register_type::<CameraActions>()
            .register_type::<UiActions>()
            .init_resource::<Actions>()
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(set_actions.label("set_actions")),
            );
    }
}

#[derive(Debug, Clone, Reflect, FromReflect, Default, Resource)]
#[reflect(Resource)]
pub struct Actions {
    pub player: PlayerActions,
    pub camera: CameraActions,
    pub ui: UiActions,
}

#[derive(Debug, Clone, Reflect, FromReflect, Default)]
pub struct PlayerActions {
    pub movement: Option<Vec2>,
    pub interact: bool,
    pub jump: bool,
    pub sprint: bool,
}

#[derive(Debug, Clone, Reflect, FromReflect, Default)]
pub struct CameraActions {
    pub movement: Option<Vec2>,
}

#[derive(Debug, Clone, Reflect, FromReflect, Default)]
pub struct UiActions {
    pub toggle_editor: bool,
}

pub fn set_actions(
    mut actions: ResMut<Actions>,
    keyboard_input: Res<Input<ScanCode>>,
    mut mouse_motion: EventReader<MouseMotion>,
    actions_frozen: Option<Res<ActionsFrozen>>,
) {
    actions.ui.toggle_editor = GameControl::ToggleEditor.just_pressed(&keyboard_input);
    if actions_frozen.is_some() {
        *actions = Actions {
            ui: UiActions {
                toggle_editor: actions.ui.toggle_editor,
                ..default()
            },
            ..default()
        };
        return;
    }

    let player_movement = Vec2::new(
        get_movement(GameControl::Right, &keyboard_input)
            - get_movement(GameControl::Left, &keyboard_input),
        get_movement(GameControl::Up, &keyboard_input)
            - get_movement(GameControl::Down, &keyboard_input),
    );

    if player_movement != Vec2::ZERO {
        actions.player.movement = Some(player_movement.normalize());
    } else {
        actions.player.movement = None;
    }
    actions.player.jump = get_movement(GameControl::Jump, &keyboard_input) > 0.5;
    actions.player.sprint = get_movement(GameControl::Sprint, &keyboard_input) > 0.5;
    actions.player.interact = GameControl::Interact.just_pressed(&keyboard_input);

    actions.camera.movement = None;
    for event in mouse_motion.iter() {
        actions.camera.movement = Some(event.delta)
    }
}
