use bevy::prelude::{Input, Res, ScanCode};

pub enum GameControl {
    Up,
    Down,
    Left,
    Right,
    Jump,
    ToggleEditor,
    Interact,
}

macro_rules! generate_bindings {
    ( $( $game_control:pat => $key_codes:expr, )+ ) => {
        impl GameControl {
            #[allow(dead_code)]
            fn just_released(&self, keyboard_input: &Res<Input<ScanCode>>) -> bool {
                match self {
                    $ (
                        $game_control => keyboard_input.any_just_released($key_codes),
                    )+
                }
            }

            #[allow(dead_code)]
            fn just_pressed(&self, keyboard_input: &Res<Input<ScanCode>>) -> bool {
                match self {
                    $ (
                        $game_control => keyboard_input.any_just_pressed($key_codes),
                    )+
                }
            }

            fn pressed(&self, keyboard_input: &Res<Input<ScanCode>>) -> bool {
                match self {
                    $ (
                        $game_control => keyboard_input.any_pressed($key_codes),
                    )+
                }
            }
        }
    };
    ( $( $game_control:pat => $key_codes:expr ),+ ) => {
        generate_bindings!($($game_control => $key_codes,)+);
    };
}

pub fn get_movement(control: GameControl, input: &Res<Input<ScanCode>>) -> f32 {
    if control.pressed(input) {
        1.0
    } else {
        0.0
    }
}

generate_bindings! {
    GameControl::Up => [
        ScanCode(13), // W
        ScanCode(126), // Up arrow
    ],
    GameControl::Down => [
        ScanCode(1), // S
        ScanCode(125), // Down arrow
    ],
    GameControl::Left => [
        ScanCode(0), // A
        ScanCode(123), // Left arrow
    ],
    GameControl::Right => [
        ScanCode(2), // D
        ScanCode(124), // Right arrow
    ],
    GameControl::Jump => [
        ScanCode(49), // Space
    ],
    GameControl::ToggleEditor => [
        ScanCode(12), // Q
    ],
    GameControl::Interact => [
        ScanCode(14), // E
    ],
}
