use crate::player_control::camera::kind::update_drivers;
use crate::player_control::camera::{
    cursor::grab_cursor, focus::set_camera_focus, kind::update_kind, rig::update_rig,
};
use crate::GameState;
use bevy::prelude::*;
use bevy_atmosphere::prelude::*;
use bevy_dolly::prelude::*;
use bevy_xpbd_3d::PhysicsSet;
use bevy_yarnspinner_example_dialogue_view::ExampleYarnSpinnerDialogueViewSystemSet;
pub(crate) use cursor::ForceCursorGrabMode;
use serde::{Deserialize, Serialize};
use ui::*;

mod cursor;
pub(crate) mod focus;
mod kind;
mod rig;
mod ui;

#[derive(Debug, Clone, PartialEq, Component, Reflect, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Reflect, Serialize, Deserialize, Default)]
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
    app.add_plugins(AtmospherePlugin)
        .register_type::<UiCamera>()
        .register_type::<IngameCamera>()
        .register_type::<IngameCameraKind>()
        .init_resource::<ForceCursorGrabMode>()
        .add_systems(Update, Dolly::<IngameCamera>::update_active)
        .add_systems(Startup, spawn_ui_camera)
        .add_systems(OnEnter(GameState::Playing), despawn_ui_camera)
        .add_systems(Update, grab_cursor.run_if(in_state(GameState::Playing)))
        .add_systems(
            Update,
            (
                update_kind,
                update_drivers,
                set_camera_focus.after(ExampleYarnSpinnerDialogueViewSystemSet),
                update_rig,
            )
                .chain()
                .in_set(CameraUpdateSystemSet)
                .after(PhysicsSet::Sync)
                .run_if(in_state(GameState::Playing)),
        );
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub(crate) struct CameraUpdateSystemSet;
