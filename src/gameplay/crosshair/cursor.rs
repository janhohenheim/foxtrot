use std::any::Any;

use bevy::{prelude::*, window::CursorGrabMode};
use bevy_yarnspinner::events::DialogueStartEvent;

use crate::{screens::Screen, third_party::bevy_yarnspinner::is_dialogue_running};

use super::CrosshairState;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (
            capture_cursor
                .param_warn_once()
                .run_if(not(is_dialogue_running)),
            release_cursor
                .param_warn_once()
                .run_if(on_event::<DialogueStartEvent>),
        )
            .chain()
            .run_if(in_state(Screen::Gameplay)),
    );
    app.add_systems(OnExit(Screen::Gameplay), release_cursor.param_warn_once());
}

fn capture_cursor(
    mut window: Single<&mut Window>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
    mut crosshair: Single<&mut CrosshairState>,
) {
    window.cursor_options.grab_mode = CursorGrabMode::Locked;
    window.cursor_options.visible = false;
    if mouse_button_input.just_pressed(MouseButton::Left) {
        // Clear Bevy's grab mode cache by setting a different grab mode
        // because an unlocked cursor will not update the current `CursorGrabMode`.
        // See <https://github.com/bevyengine/bevy/issues/8949>
        window.cursor_options.grab_mode = CursorGrabMode::Confined;
    }
    crosshair.wants_invisible.remove(&release_cursor.type_id());
}

pub fn release_cursor(
    mut window: Single<&mut Window>,
    crosshair: Option<Single<&mut CrosshairState>>,
) {
    window.cursor_options.visible = true;
    window.cursor_options.grab_mode = CursorGrabMode::None;
    if let Some(mut crosshair) = crosshair {
        crosshair.wants_invisible.insert(release_cursor.type_id());
    }
}
