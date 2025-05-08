//! Handle cursor capture and release. This is a bit hacky because winit does not have a nice way to keep the cursor locked in a browser.

use std::any::Any;

use bevy::{input::common_conditions::input_just_pressed, prelude::*, window::CursorGrabMode};

use crate::{AppSystems, screens::Screen, third_party::bevy_yarnspinner::is_dialogue_running};

use super::CrosshairState;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<IsCursorForceUnlocked>();
    app.add_systems(
        Update,
        (
            capture_cursor_delayed.run_if(resource_exists::<CaptureCursorDelayed>),
            capture_cursor.run_if(
                input_just_pressed(MouseButton::Left)
                    .and(not(is_dialogue_running))
                    .and(not(is_cursor_force_unlocked))
                    .and(in_state(Screen::Gameplay).or(in_state(Screen::Loading))),
            ),
            release_cursor.run_if(is_cursor_force_unlocked),
            on_cursor_lock_changed,
        )
            .chain()
            .in_set(AppSystems::ChangeUi),
    );
    app.add_systems(OnExit(Screen::Gameplay), release_cursor);
}

fn capture_cursor(mut window: Single<&mut Window>, mut commands: Commands) {
    // Need to clear Bevy's cache because in some cases the `CursorGrabMode` is set to `Locked` even though the cursor is not actually locked.
    // But setting it to `Locked` while in this state does not do anything, due to Bevy's cache!
    // So we need to clear the cache first with this useless call.
    window.cursor_options.grab_mode = CursorGrabMode::Confined;
    commands.insert_resource(CaptureCursorDelayed);
}

fn capture_cursor_delayed(mut window: Single<&mut Window>, mut commands: Commands) {
    // Set the *actual* cursor mode one frame after the `Confined` mode has been set.
    window.cursor_options.grab_mode = CursorGrabMode::Locked;
    commands.remove_resource::<CaptureCursorDelayed>();
}

#[derive(Resource)]
struct CaptureCursorDelayed;

fn release_cursor(mut window: Single<&mut Window>) {
    window.cursor_options.grab_mode = CursorGrabMode::None;
}

fn on_cursor_lock_changed(
    #[cfg_attr(not(feature = "native"), allow(unused_mut))] mut window: Single<
        &mut Window,
        Changed<Window>,
    >,
    crosshair: Option<Single<&mut CrosshairState>>,
    mut last_grab_mode: Local<CursorGrabMode>,
) {
    let grab_mode = window.cursor_options.grab_mode;
    if grab_mode == *last_grab_mode {
        return;
    }
    *last_grab_mode = grab_mode;
    if grab_mode == CursorGrabMode::None {
        if let Some(mut crosshair) = crosshair {
            crosshair.wants_invisible.insert(release_cursor.type_id());
        }
        #[cfg(feature = "native")]
        {
            window.cursor_options.visible = true;
        }
    } else {
        if let Some(mut crosshair) = crosshair {
            crosshair.wants_invisible.remove(&release_cursor.type_id());
        }
        #[cfg(feature = "native")]
        {
            window.cursor_options.visible = false;
        }
    }
}

/// A resource that indicates whether the cursor cannot be captured.
/// This is useful for e.g. debug menus.
#[derive(Resource, Debug, Default, Reflect)]
#[reflect(Resource)]
pub(crate) struct IsCursorForceUnlocked(pub(crate) bool);

fn is_cursor_force_unlocked(val: Res<IsCursorForceUnlocked>) -> bool {
    val.0
}
