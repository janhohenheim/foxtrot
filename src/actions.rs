use bevy::{log, prelude::*};
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
