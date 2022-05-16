use crate::networking::protocol::{InputFlags, InputProtocol, LocalHandles};
use bevy::{log, prelude::*};
use ggrs::{InputStatus, PlayerHandle};
pub struct ActionsPlugin;

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for ActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Vec<Actions>>();
    }
}

#[derive(Debug, Component, Reflect, Default, Clone)]
pub struct Actions {
    pub player_movement: Option<Vec2>,
}

pub fn set_movement_actions(
    mut actions: ResMut<Vec<Actions>>,
    inputs: Res<Vec<(InputProtocol, InputStatus)>>,
) {
    *actions = inputs
        .iter()
        .map(|(protocol, status)| parse_protocol_to_actions(protocol, *status))
        .collect();
}

fn parse_protocol_to_actions(protocol: &InputProtocol, status: InputStatus) -> Actions {
    let mut action = Actions::default();
    if status == InputStatus::Disconnected {
        log::warn!("Player disconnected");
        return action;
    }

    let input = InputFlags::try_from(*protocol).unwrap();
    if input.is_empty() {
        return action;
    }

    let mut player_movement = Vec2::ZERO;
    if input.contains(InputFlags::LEFT) {
        player_movement.x -= 1.;
    }
    if input.contains(InputFlags::RIGHT) {
        player_movement.x += 1.;
    }
    if input.contains(InputFlags::UP) {
        player_movement.y += 1.;
    }
    if input.contains(InputFlags::DOWN) {
        player_movement.y -= 1.;
    }

    if player_movement == Vec2::ZERO {
        return action;
    }

    player_movement = player_movement.normalize();
    action.player_movement = Some(player_movement);
    action
}

enum GameControl {
    Up,
    Down,
    Left,
    Right,
    Fire,
}

macro_rules! generate_bindings {
    ( $( $game_control:pat => $key_codes:expr ),+ ) => {

            impl GameControl {
                #[allow(dead_code)]
                fn just_released(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
                    match self {
                        $ (
                            $game_control => keyboard_input.any_just_released($key_codes),
                        )+
                    }
                }

                #[allow(dead_code)]
                fn just_pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
                    match self {
                        $ (
                            $game_control => keyboard_input.any_just_pressed($key_codes),
                        )+
                    }
                }

                fn pressed(&self, keyboard_input: &Res<Input<KeyCode>>) -> bool {
                    match self {
                        $ (
                            $game_control => keyboard_input.any_pressed($key_codes),
                        )+
                    }
                }
            }

    };
}

generate_bindings! {
    GameControl::Up => [KeyCode::W, KeyCode::Up,],
    GameControl::Down => [KeyCode::S, KeyCode::Down,],
    GameControl::Left => [KeyCode::A, KeyCode::Left,],
    GameControl::Right => [KeyCode::D, KeyCode::Right,],
    GameControl::Fire => [KeyCode::Space, KeyCode::Return]
}

pub fn create_input_protocol(
    _handle: In<PlayerHandle>,
    keyboard_input: Res<Input<KeyCode>>,
    _local_handles: Res<LocalHandles>,
) -> InputProtocol {
    let mut input = InputFlags::empty();

    if GameControl::Up.pressed(&keyboard_input) {
        input |= InputFlags::UP;
    }
    if GameControl::Down.pressed(&keyboard_input) {
        input |= InputFlags::DOWN;
    }
    if GameControl::Left.pressed(&keyboard_input) {
        input |= InputFlags::LEFT;
    }
    if GameControl::Right.pressed(&keyboard_input) {
        input |= InputFlags::RIGHT;
    }
    if GameControl::Fire.pressed(&keyboard_input) {
        input |= InputFlags::FIRE;
    }

    input.into()
}
