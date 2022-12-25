use bevy::prelude::{Input, KeyCode, Res};

pub enum GameControl {
    Up,
    Down,
    Left,
    Right,
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

pub fn get_movement(control: GameControl, input: &Res<Input<KeyCode>>) -> f32 {
    if control.pressed(input) {
        1.0
    } else {
        0.0
    }
}

generate_bindings! {
    GameControl::Up => [KeyCode::W, KeyCode::Up,],
    GameControl::Down => [KeyCode::S, KeyCode::Down,],
    GameControl::Left => [KeyCode::A, KeyCode::Left,],
    GameControl::Right => [KeyCode::D, KeyCode::Right,]
}
