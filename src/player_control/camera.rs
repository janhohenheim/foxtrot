use crate::player_control::camera::{
    cursor::grab_cursor, focus::set_camera_focus, kind::update_kind, rig::update_rig,
    skydome::move_skydome,
};
use crate::util::log_error::log_errors;
use crate::GameState;
use bevy::prelude::*;
use bevy_dolly::prelude::*;
pub use cursor::ForceCursorGrabMode;
use serde::{Deserialize, Serialize};
use ui::*;

mod cursor;
pub mod focus;
mod kind;
mod rig;
mod skydome;
mod ui;
mod util;

#[derive(Debug, Clone, PartialEq, Component, Reflect, Serialize, Deserialize, FromReflect)]
#[reflect(Component, Serialize, Deserialize)]
pub struct IngameCamera {
    pub target: Transform,
    pub desired_distance: f32,
    pub kind: IngameCameraKind,
}

impl Default for IngameCamera {
    fn default() -> Self {
        Self {
            target: default(),
            desired_distance: 5.,
            kind: default(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Reflect, FromReflect, Serialize, Deserialize, Default)]
#[reflect(Serialize, Deserialize)]
pub enum IngameCameraKind {
    #[default]
    ThirdPerson,
    FirstPerson,
    FixedAngle,
}

/// Handles the main ingame camera, i.e. not the UI camera in the menu.
/// Cameras are controlled with [`CameraActions`]. Depending on the distance, a first person,
/// third person or fixed angle camera is used.
pub struct CameraPlugin;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct SetCameraFocusLabel;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<UiCamera>()
            .register_type::<IngameCamera>()
            .register_type::<IngameCameraKind>()
            .init_resource::<ForceCursorGrabMode>()
            .add_system(Dolly::<IngameCamera>::update_active)
            .add_system(spawn_ui_camera.on_startup())
            .add_system(despawn_ui_camera.in_schedule(OnEnter(GameState::Playing)))
            .add_systems((grab_cursor.pipe(log_errors),).in_set(OnUpdate(GameState::Playing)))
            .add_systems(
                (
                    update_kind,
                    set_camera_focus
                        .pipe(log_errors)
                        .in_set(SetCameraFocusLabel),
                    update_rig.pipe(log_errors),
                    move_skydome,
                )
                    .chain()
                    .in_set(CameraUpdateSystemSet)
                    .in_set(OnUpdate(GameState::Playing)),
            );
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct CameraUpdateSystemSet;
