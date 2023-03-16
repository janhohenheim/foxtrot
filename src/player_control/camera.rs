use crate::level_instantiation::spawning::objects::skydome::Skydome;
use crate::player_control::actions::ActionsFrozen;
use crate::player_control::camera::focus::set_camera_focus;
use crate::util::log_error::log_errors;
use crate::GameState;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use bevy::window::PrimaryWindow;
use bevy_dolly::prelude::*;
use serde::{Deserialize, Serialize};
use ui::*;

pub mod focus;
mod third_person;
mod ui;
mod util;

#[derive(Debug, Clone, PartialEq, Component, Reflect, Serialize, Deserialize, FromReflect)]
#[reflect(Component, Serialize, Deserialize)]
pub struct IngameCamera {
    pub target: Vec3,
    pub secondary_target: Option<Vec3>,
    pub desired_distance: f32,
    pub kind: IngameCameraKind,
}

impl Default for IngameCamera {
    fn default() -> Self {
        Self {
            target: default(),
            secondary_target: default(),
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
            .add_systems(
                (cursor_grab_system.pipe(log_errors),).in_set(OnUpdate(GameState::Playing)),
            )
            .add_systems(
                (
                    set_camera_focus
                        .pipe(log_errors)
                        .in_set(SetCameraFocusLabel),
                    third_person::update_rig.pipe(log_errors),
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

fn move_skydome(
    camera_query: Query<&Transform, (With<IngameCamera>, Without<Skydome>)>,
    mut skydome_query: Query<&mut Transform, (Without<IngameCamera>, With<Skydome>)>,
) {
    #[cfg(feature = "tracing")]
    let _span = info_span!("move_skydome").entered();
    for camera_transform in camera_query.iter() {
        for mut skydome_transform in skydome_query.iter_mut() {
            skydome_transform.translation = camera_transform.translation;
        }
    }
}

#[derive(Debug, Clone, Copy, Resource, Serialize, Deserialize, Default)]
pub struct ForceCursorGrabMode(pub Option<CursorGrabMode>);

fn cursor_grab_system(
    mut primary_windows: Query<&mut Window, With<PrimaryWindow>>,
    actions_frozen: Res<ActionsFrozen>,
    force_cursor_grab: Res<ForceCursorGrabMode>,
) -> Result<()> {
    #[cfg(feature = "tracing")]
    let _span = info_span!("cursor_grab_system").entered();
    let mut window = primary_windows
        .get_single_mut()
        .context("Failed to get primary window")?;
    let cursor = &mut window.cursor;
    if let Some(mode) = force_cursor_grab.0 {
        cursor.grab_mode = mode;
        cursor.visible = mode != CursorGrabMode::Locked;
    } else if actions_frozen.is_frozen() {
        cursor.grab_mode = CursorGrabMode::None;
        cursor.visible = true;
    } else {
        cursor.grab_mode = CursorGrabMode::Locked;
        cursor.visible = false;
    }
    Ok(())
}
