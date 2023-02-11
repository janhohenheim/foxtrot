use crate::player_control::actions::ActionsFrozen;
use crate::player_control::camera::focus::set_camera_focus;
use crate::GameState;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;
pub use third_person::ThirdPersonCamera;
use third_person::*;
use ui::*;

pub mod focus;
mod third_person;
mod ui;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<UiCamera>()
            .register_type::<ThirdPersonCamera>()
            .add_startup_system(spawn_ui_camera)
            // Enables the system that synchronizes your `Transform`s and `LookTransform`s.
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(despawn_ui_camera))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(follow_target.label("follow_target"))
                    .with_system(
                        handle_camera_controls
                            .label("handle_camera_controls")
                            .after("follow_target"),
                    )
                    .with_system(
                        update_camera_transform
                            .label("update_camera_transform")
                            .after("handle_camera_controls"),
                    )
                    .with_system(cursor_grab_system)
                    .with_system(init_camera_eye.before("follow_target"))
                    .with_system(set_camera_focus.before("follow_target")),
            );
    }
}

fn cursor_grab_system(
    mut windows: ResMut<Windows>,
    key: Res<Input<KeyCode>>,
    frozen: Option<Res<ActionsFrozen>>,
) {
    let window = windows.get_primary_mut().unwrap();
    if frozen.is_some() {
        window.set_cursor_grab_mode(CursorGrabMode::None);
        window.set_cursor_visibility(true);
        return;
    }
    if key.just_pressed(KeyCode::Escape) {
        if matches!(window.cursor_grab_mode(), CursorGrabMode::None) {
            // if you want to use the cursor, but not let it leave the window,
            // use `Confined` mode:
            window.set_cursor_grab_mode(CursorGrabMode::Confined);

            // for a game that doesn't use the cursor (like a shooter):
            // use `Locked` mode to keep the cursor in one place
            window.set_cursor_grab_mode(CursorGrabMode::Locked);
            // also hide the cursor
            window.set_cursor_visibility(false);
        } else {
            window.set_cursor_grab_mode(CursorGrabMode::None);
            window.set_cursor_visibility(true);
        }
    }
}
