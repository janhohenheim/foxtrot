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
        app.init_resource::<Actions>()
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(set_actions));
    }
}

#[derive(Default, Resource)]
pub struct Actions {
    pub player_movement: Option<Vec2>,
    pub camera_movement: Option<Vec2>,
    pub toggle_editor: bool,
    pub interact: bool,
    pub jump: bool,
}

pub fn set_actions(
    mut actions: ResMut<Actions>,
    keyboard_input: Res<Input<ScanCode>>,
    mut mouse_input: EventReader<MouseMotion>,
    actions_frozen: Option<Res<ActionsFrozen>>,
) {
    actions.toggle_editor = GameControl::ToggleEditor.just_pressed(&keyboard_input);
    if actions_frozen.is_some() {
        *actions = Actions {
            toggle_editor: actions.toggle_editor,
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
        actions.player_movement = Some(player_movement.normalize());
    } else {
        actions.player_movement = None;
    }
    actions.jump = get_movement(GameControl::Jump, &keyboard_input) > 0.5;
    actions.interact = GameControl::Interact.just_pressed(&keyboard_input);

    actions.camera_movement = None;
    for event in mouse_input.iter() {
        actions.camera_movement = Some(event.delta)
    }
}
