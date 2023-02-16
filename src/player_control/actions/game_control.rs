use bevy::prelude::{Input, Res, ScanCode};

pub enum GameControl {
    Up,
    Down,
    Left,
    Right,
    Sprint,
    Jump,
    ToggleEditor,
    Interact,
    NumberedChoice(u16),
}

macro_rules! generate_bindings {
    ( $( $game_control:pat => $key_codes:expr, )+ ) => {
        impl GameControl {
            #[allow(dead_code)]
            pub fn just_released(&self, keyboard_input: &Res<Input<ScanCode>>) -> bool {
                match self {
                    $ (
                        $game_control => keyboard_input.any_just_released($key_codes),
                    )+
                }
            }

            #[allow(dead_code)]
            pub fn just_pressed(&self, keyboard_input: &Res<Input<ScanCode>>) -> bool {
                match self {
                    $ (
                        $game_control => keyboard_input.any_just_pressed($key_codes),
                    )+
                }
            }

            pub fn pressed(&self, keyboard_input: &Res<Input<ScanCode>>) -> bool {
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

// MacOS: sampled by hand
// Windows: <https://superuser.com/a/1454198>
// Linux: <http://www.quadibloc.com/comp/scan.htm>
generate_bindings! {
    GameControl::Up => [
         // W
        ScanCode(
            #[cfg(target_os = "macos")] 13,
            #[cfg(target_os = "windows")] 0x11,
            #[cfg(target_os = "linux")] 0x11,
            #[cfg(target_arch = "wasm32")] 0x11,
        ),
        // Up arrow
        ScanCode(
            #[cfg(target_os = "macos")] 126,
            #[cfg(target_os = "windows")] 0x48,
            #[cfg(target_os = "linux")] 0x48,
            #[cfg(target_arch = "wasm32")] 0x48,
        ),
    ],
    GameControl::Down => [
        // S
        ScanCode(
            #[cfg(target_os = "macos")] 1,
            #[cfg(target_os = "windows")] 0x1F,
            #[cfg(target_os = "linux")] 0x1F,
            #[cfg(target_arch = "wasm32")] 0x1F,
        ),
        // Down arrow
        ScanCode(
            #[cfg(target_os = "macos")] 125,
            #[cfg(target_os = "windows")] 0x50,
            #[cfg(target_os = "linux")] 0x50,
            #[cfg(target_arch = "wasm32")] 0x50,
        ),
    ],
    GameControl::Left => [
        // A
        ScanCode(
            #[cfg(target_os = "macos")] 0,
            #[cfg(target_os = "windows")] 0x1E,
            #[cfg(target_os = "linux")] 0x1E,
            #[cfg(target_arch = "wasm32")] 0x1E,
        ),
        // Left arrow
        ScanCode(
            #[cfg(target_os = "macos")] 123,
            #[cfg(target_os = "windows")] 0x4B,
            #[cfg(target_os = "linux")] 0x4B,
            #[cfg(target_arch = "wasm32")] 0x4B,
        ),
    ],
    GameControl::Right => [
        // D
        ScanCode(
            #[cfg(target_os = "macos")] 2,
            #[cfg(target_os = "windows")] 0x20,
            #[cfg(target_os = "linux")] 0x20,
            #[cfg(target_arch = "wasm32")] 0x20,
        ),
        // Right arrow
        ScanCode(
            #[cfg(target_os = "macos")] 124,
            #[cfg(target_os = "windows")] 0x4D,
            #[cfg(target_os = "linux")] 0x4D,
            #[cfg(target_arch = "wasm32")] 0x4D,
        ),
    ],
    GameControl::Sprint => [
        // Left shift
        ScanCode(
            #[cfg(target_os = "macos")] 56,
            #[cfg(target_os = "windows")] 0x2A,
            #[cfg(target_os = "linux")] 0x2A,
            #[cfg(target_arch = "wasm32")] 0x2A,
        ),
    ],
    GameControl::Jump => [
        // Space
        ScanCode(
            #[cfg(target_os = "macos")] 49,
            #[cfg(target_os = "windows")] 0x39,
            #[cfg(target_os = "linux")] 0x39,
            #[cfg(target_arch = "wasm32")] 0x39,
        ),
    ],
    GameControl::ToggleEditor => [
        // Q
        ScanCode(
            #[cfg(target_os = "macos")] 12,
            #[cfg(target_os = "windows")] 0x10,
            #[cfg(target_os = "linux")] 0x10,
            #[cfg(target_arch = "wasm32")] 0x10,
        ),
        // Esc
        ScanCode(
            #[cfg(target_os = "macos")] 53,
            #[cfg(target_os = "windows")] 0x01,
            #[cfg(target_os = "linux")] 0x01,
            #[cfg(target_arch = "wasm32")] 0x01,
        ),
    ],
    GameControl::Interact => [
        // E
        ScanCode(
            #[cfg(target_os = "macos")] 14,
            #[cfg(target_os = "windows")] 0x12,
            #[cfg(target_os = "linux")] 0x12,
            #[cfg(target_arch = "wasm32")] 0x12,
        ),
    ],
    GameControl::NumberedChoice(1) => [
        // Number 1
        ScanCode(
            #[cfg(target_os = "macos")] 18,
            #[cfg(target_os = "windows")] 0x02,
            #[cfg(target_os = "linux")] 0x02,
            #[cfg(target_arch = "wasm32")] 0x02,
        ),
        // Numpad 1
        ScanCode(
            #[cfg(target_os = "macos")] 83,
            #[cfg(target_os = "windows")] 0x4F,
            #[cfg(target_os = "linux")] 0x4F,
            #[cfg(target_arch = "wasm32")] 0x4F,
        ),
    ],
    GameControl::NumberedChoice(2) => [
        // Number 2
        ScanCode(
            #[cfg(target_os = "macos")] 19,
            #[cfg(target_os = "windows")] 0x03,
            #[cfg(target_os = "linux")] 0x03,
            #[cfg(target_arch = "wasm32")] 0x03,
        ),
        // Numpad 2
        ScanCode(
            #[cfg(target_os = "macos")] 84,
            #[cfg(target_os = "windows")] 0x50,
            #[cfg(target_os = "linux")] 0x50,
            #[cfg(target_arch = "wasm32")] 0x50,
        ),
    ],
    GameControl::NumberedChoice(3) => [
        // Number 3
        ScanCode(
            #[cfg(target_os = "macos")] 20,
            #[cfg(target_os = "windows")] 0x04,
            #[cfg(target_os = "linux")] 0x04,
            #[cfg(target_arch = "wasm32")] 0x04,
        ),
        // Numpad 3
        ScanCode(
            #[cfg(target_os = "macos")] 85,
            #[cfg(target_os = "windows")] 0x51,
            #[cfg(target_os = "linux")] 0x51,
            #[cfg(target_arch = "wasm32")] 0x51,
        ),
    ],
    GameControl::NumberedChoice(4) => [
        // Number 4
        ScanCode(
            #[cfg(target_os = "macos")] 21,
            #[cfg(target_os = "windows")] 0x05,
            #[cfg(target_os = "linux")] 0x05,
            #[cfg(target_arch = "wasm32")] 0x05,
        ),
        // Numpad 4
        ScanCode(
            #[cfg(target_os = "macos")] 86,
            #[cfg(target_os = "windows")] 0x4B,
            #[cfg(target_os = "linux")] 0x4B,
            #[cfg(target_arch = "wasm32")] 0x4B,
        ),
    ],
    GameControl::NumberedChoice(5) => [
        // Number 5
        ScanCode(
            #[cfg(target_os = "macos")] 23,
            #[cfg(target_os = "windows")] 0x06,
            #[cfg(target_os = "linux")] 0x06,
            #[cfg(target_arch = "wasm32")] 0x06,
        ),
        // Numpad 5
        ScanCode(
            #[cfg(target_os = "macos")] 87,
            #[cfg(target_os = "windows")] 0x4C,
            #[cfg(target_os = "linux")] 0x4C,
            #[cfg(target_arch = "wasm32")] 0x4C,
        ),
    ],
    GameControl::NumberedChoice(6) => [
        // Number 6
        ScanCode(
            #[cfg(target_os = "macos")] 22,
            #[cfg(target_os = "windows")] 0x07,
            #[cfg(target_os = "linux")] 0x07,
            #[cfg(target_arch = "wasm32")] 0x07,
        ),
        // Numpad 6
        ScanCode(
            #[cfg(target_os = "macos")] 88,
            #[cfg(target_os = "windows")] 0x4D,
            #[cfg(target_os = "linux")] 0x4D,
            #[cfg(target_arch = "wasm32")] 0x4D,
        ),
    ],
    GameControl::NumberedChoice(7) => [
        // Number 7
        ScanCode(
            #[cfg(target_os = "macos")] 26,
            #[cfg(target_os = "windows")] 0x08,
            #[cfg(target_os = "linux")] 0x08,
            #[cfg(target_arch = "wasm32")] 0x08,
        ),
        // Numpad 7
        ScanCode(
            #[cfg(target_os = "macos")] 89,
            #[cfg(target_os = "windows")] 0x47,
            #[cfg(target_os = "linux")] 0x47,
            #[cfg(target_arch = "wasm32")] 0x47,
        ),
    ],
    GameControl::NumberedChoice(8) => [
        // Number 8
        ScanCode(
            #[cfg(target_os = "macos")] 28,
            #[cfg(target_os = "windows")] 0x09,
            #[cfg(target_os = "linux")] 0x09,
            #[cfg(target_arch = "wasm32")] 0x09,
        ),
        // Numpad 8
        ScanCode(
            #[cfg(target_os = "macos")] 91,
            #[cfg(target_os = "windows")] 0x48,
            #[cfg(target_os = "linux")] 0x48,
            #[cfg(target_arch = "wasm32")] 0x48,
        ),
    ],
    GameControl::NumberedChoice(9) => [
        // Number 9
        ScanCode(
            #[cfg(target_os = "macos")] 25,
            #[cfg(target_os = "windows")] 0x0A,
            #[cfg(target_os = "linux")] 0x0A,
            #[cfg(target_arch = "wasm32")] 0x0A,
        ),
        // Numpad 9
        ScanCode(
            #[cfg(target_os = "macos")] 92,
            #[cfg(target_os = "windows")] 0x49,
            #[cfg(target_os = "linux")] 0x49,
            #[cfg(target_arch = "wasm32")] 0x49,
        ),
    ],
    GameControl::NumberedChoice(0) => [
        // Number 0
        ScanCode(
            #[cfg(target_os = "macos")] 29,
            #[cfg(target_os = "windows")] 0x0B,
            #[cfg(target_os = "linux")] 0x0B,
            #[cfg(target_arch = "wasm32")] 0x0B,
        ),
        // Numpad 0
        ScanCode(
            #[cfg(target_os = "macos")] 82,
            #[cfg(target_os = "windows")] 0x52,
            #[cfg(target_os = "linux")] 0x52,
            #[cfg(target_arch = "wasm32")] 0x52,
        ),
    ],
    GameControl::NumberedChoice(_) => [
        // Unreachable
        ScanCode(
            #[cfg(target_os = "macos")] 0,
            #[cfg(target_os = "windows")] 0x00,
            #[cfg(target_os = "linux")] 0x00,
            #[cfg(target_arch = "wasm32")] 0x00,
        ),
    ],
}
