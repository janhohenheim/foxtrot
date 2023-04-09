use crate::player_control::camera::kind::update_drivers;
use crate::player_control::camera::{
    cursor::grab_cursor, focus::set_camera_focus, kind::update_kind, rig::update_rig,
    skydome::move_skydome,
};
use crate::GameState;
use bevy::prelude::*;
use bevy_dolly::prelude::*;
pub(crate) use cursor::ForceCursorGrabMode;
use serde::{Deserialize, Serialize};
use ui::*;

mod cursor;
pub(crate) mod focus;
mod kind;
mod rig;
mod skydome;
mod ui;

#[derive(Debug, Clone, PartialEq, Component, Reflect, Serialize, Deserialize, FromReflect)]
#[reflect(Component, Serialize, Deserialize)]
pub(crate) struct IngameCamera {
    pub(crate) target: Transform,
    pub(crate) secondary_target: Option<Transform>,
    pub(crate) desired_distance: f32,
    pub(crate) kind: IngameCameraKind,
}

impl Default for IngameCamera {
    fn default() -> Self {
        Self {
            desired_distance: 5.,
            target: default(),
            secondary_target: default(),
            kind: default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Reflect, FromReflect, Serialize, Deserialize, Default)]
#[reflect(Serialize, Deserialize)]
pub(crate) enum IngameCameraKind {
    #[default]
    ThirdPerson,
    FirstPerson,
    FixedAngle,
}

/// Handles the main ingame camera, i.e. not the UI camera in the menu.
/// Cameras are controlled with [`CameraActions`]. Depending on the distance, a first person,
/// third person or fixed angle camera is used.
pub(crate) fn camera_plugin(app: &mut App) {
    app.register_type::<UiCamera>()
        .register_type::<IngameCamera>()
        .register_type::<IngameCameraKind>()
        .init_resource::<ForceCursorGrabMode>()
        .add_system(Dolly::<IngameCamera>::update_active)
        .add_system(spawn_ui_camera.on_startup())
        .add_system(despawn_ui_camera.in_schedule(OnEnter(GameState::Playing)))
        .add_system(grab_cursor.in_set(OnUpdate(GameState::Playing)))
        .add_systems(
            (
                update_kind,
                update_drivers,
                set_camera_focus,
                update_rig,
                move_skydome,
            )
                .chain()
                .in_set(CameraUpdateSystemSet)
                .in_set(OnUpdate(GameState::Playing)),
        );
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub(crate) struct CameraUpdateSystemSet;
