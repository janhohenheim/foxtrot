use bevy::{prelude::*, window::CursorGrabMode};

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, capture_cursor.run_if(in_state(Screen::Gameplay)));
    app.add_systems(OnExit(Screen::Gameplay), release_cursor);
}

fn capture_cursor(
    mut window: Single<&mut Window>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) {
    window.cursor_options.grab_mode = CursorGrabMode::Locked;
    window.cursor_options.visible = false;
    if mouse_button_input.just_pressed(MouseButton::Left) {
        // Clear Bevy's grab mode cache by setting a different grab mode
        // because an unlocked cursor will not update the current `CursorGrabMode`.
        // See <https://github.com/bevyengine/bevy/issues/8949>
        window.cursor_options.grab_mode = CursorGrabMode::Confined;
    }
}

fn release_cursor(mut window: Single<&mut Window>) {
    window.cursor_options.visible = true;
    window.cursor_options.grab_mode = CursorGrabMode::None;
}
