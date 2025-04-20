//! Handle cursor capture and release. This is a bit hacky because winit does not have a nice way to keep the cursor locked in a browser.

use std::any::Any;

use bevy::{prelude::*, window::CursorGrabMode};
use bevy_yarnspinner::events::DialogueStartEvent;

use crate::{AppSet, screens::Screen, third_party::bevy_yarnspinner::is_dialogue_running};

use super::CrosshairState;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<IsCursorForcedFreed>();
    app.add_systems(
        Update,
        (
            capture_cursor
                .param_warn_once()
                .run_if(not(is_dialogue_running).and(not(is_cursor_forced_freed))),
            release_cursor
                .param_warn_once()
                .run_if(on_event::<DialogueStartEvent>.or(is_cursor_forced_freed)),
        )
            .chain()
            .run_if(in_state(Screen::Gameplay))
            .in_set(AppSet::ChangeUi),
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

pub fn release_cursor(mut window: Single<&mut Window>) {
    window.cursor_options.visible = true;
    window.cursor_options.grab_mode = CursorGrabMode::None;
}

pub(crate) fn is_cursor_forced_freed(val: Res<IsCursorForcedFreed>) -> bool {
    val.0
}

/// A resource that indicates whether the cursor cannot be captured.
/// This is useful for e.g. debug menus.
#[derive(Resource, Debug, Default, Reflect)]
#[reflect(Resource)]
pub(crate) struct IsCursorForcedFreed(pub(crate) bool);
