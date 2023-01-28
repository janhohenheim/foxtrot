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
        ),
        // Up arrow
        ScanCode(
            #[cfg(target_os = "macos")] 126,
            #[cfg(target_os = "windows")] 0x48,
            #[cfg(target_os = "linux")] 0x48,
        ),
    ],
    GameControl::Down => [
        // S
        ScanCode(
            #[cfg(target_os = "macos")] 1,
            #[cfg(target_os = "windows")] 0x1F,
            #[cfg(target_os = "linux")] 0x1F,
        ),
        // Down arrow
        ScanCode(
            #[cfg(target_os = "macos")] 125,
            #[cfg(target_os = "windows")] 0x50,
            #[cfg(target_os = "linux")] 0x50,
        ),
    ],
    GameControl::Left => [
        // A
        ScanCode(
            #[cfg(target_os = "macos")] 0,
            #[cfg(target_os = "windows")] 0x1E,
            #[cfg(target_os = "linux")] 0x1E,
        ),
        // Left arrow
        ScanCode(
            #[cfg(target_os = "macos")] 123,
            #[cfg(target_os = "windows")] 0x4B,
            #[cfg(target_os = "linux")] 0x4B,
        ),
    ],
    GameControl::Right => [
        // D
        ScanCode(
            #[cfg(target_os = "macos")] 2,
            #[cfg(target_os = "windows")] 0x20,
            #[cfg(target_os = "linux")] 0x20,
        ),
        // Right arrow
        ScanCode(
            #[cfg(target_os = "macos")] 124,
            #[cfg(target_os = "windows")] 0x4D,
            #[cfg(target_os = "linux")] 0x4D,
        ),
    ],
    GameControl::Jump => [
        // Space
        ScanCode(
            #[cfg(target_os = "macos")] 49,
            #[cfg(target_os = "windows")] 0x39,
            #[cfg(target_os = "linux")] 0x39,
        ),
    ],
    GameControl::ToggleEditor => [
        // Q
        ScanCode(
            #[cfg(target_os = "macos")] 12,
            #[cfg(target_os = "windows")] 0x10,
            #[cfg(target_os = "linux")] 0x10,
        ),
    ],
    GameControl::Interact => [
        // E
        ScanCode(
            #[cfg(target_os = "macos")] 14,
            #[cfg(target_os = "windows")] 0x12,
            #[cfg(target_os = "linux")] 0x12,
        ),
    ],
}
