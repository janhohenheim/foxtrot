use crate::player_control::actions::{ActionsFrozen, CameraActions};
use crate::player_control::camera::focus::{set_camera_focus, switch_kind};
use crate::player_control::player_embodiment::set_camera_actions;
use crate::util::log_error::log_errors;
use crate::GameState;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use bevy_rapier3d::prelude::*;
pub use first_person::FirstPersonCamera;
pub use fixed_angle::FixedAngleCamera;
use serde::{Deserialize, Serialize};
pub use third_person::ThirdPersonCamera;
use ui::*;

mod first_person;
mod fixed_angle;
pub mod focus;
mod third_person;
mod ui;
mod util;

#[derive(Debug, Clone, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct IngameCamera {
    pub kind: IngameCameraKind,
    pub actions: CameraActions,
}

impl IngameCamera {
    pub fn set_primary_target(&mut self, target: Vec3) {
        match &mut self.kind {
            IngameCameraKind::ThirdPerson(camera) => {
                camera.target = target;
            }
            IngameCameraKind::FirstPerson(camera) => {
                camera.transform.translation = target;
            }
            IngameCameraKind::FixedAngle(camera) => {
                camera.target = target;
            }
        }
    }

    pub fn up(&self) -> Vec3 {
        match &self.kind {
            IngameCameraKind::ThirdPerson(camera) => camera.up,
            IngameCameraKind::FirstPerson(camera) => camera.up,
            IngameCameraKind::FixedAngle(camera) => camera.up,
        }
    }

    pub fn up_mut(&mut self) -> &mut Vec3 {
        match &mut self.kind {
            IngameCameraKind::ThirdPerson(camera) => &mut camera.up,
            IngameCameraKind::FirstPerson(camera) => &mut camera.up,
            IngameCameraKind::FixedAngle(camera) => &mut camera.up,
        }
    }

    pub fn forward(&self) -> Vec3 {
        match &self.kind {
            IngameCameraKind::ThirdPerson(camera) => camera.forward(),
            IngameCameraKind::FirstPerson(camera) => camera.forward(),
            IngameCameraKind::FixedAngle(camera) => camera.forward(),
        }
    }

    pub fn secondary_target_mut(&mut self) -> &mut Option<Vec3> {
        match &mut self.kind {
            IngameCameraKind::ThirdPerson(camera) => &mut camera.secondary_target,
            IngameCameraKind::FirstPerson(camera) => &mut camera.look_target,
            IngameCameraKind::FixedAngle(camera) => &mut camera.secondary_target,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Reflect, FromReflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub enum IngameCameraKind {
    ThirdPerson(ThirdPersonCamera),
    FirstPerson(FirstPersonCamera),
    FixedAngle(FixedAngleCamera),
}

impl Default for IngameCameraKind {
    fn default() -> Self {
        Self::ThirdPerson(ThirdPersonCamera::default())
    }
}

/// Handles the main ingame camera, i.e. not the UI camera in the menu.
/// Cameras are controlled with [`CameraActions`]. Depending on the distance, a first person,
/// third person or fixed angle camera is used.
pub struct CameraPlugin;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
pub struct SetCameraFocusLabel;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<UiCamera>()
            .register_type::<ThirdPersonCamera>()
            .register_type::<IngameCamera>()
            .register_type::<IngameCameraKind>()
            .add_startup_system(spawn_ui_camera)
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(despawn_ui_camera))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(cursor_grab_system.pipe(log_errors))
                    .with_system(init_camera)
                    .with_system(set_camera_focus.pipe(log_errors).label(SetCameraFocusLabel))
                    .with_system(
                        switch_kind
                            .after(set_camera_actions)
                            .after(SetCameraFocusLabel),
                    )
                    .with_system(update_transform.after(switch_kind))
                    .with_system(reset_actions.after(update_transform)),
            );
    }
}

fn init_camera(mut camera: Query<(&Transform, &mut IngameCamera), Added<IngameCamera>>) {
    for (transform, mut camera) in camera.iter_mut() {
        match &mut camera.kind {
            IngameCameraKind::ThirdPerson(camera) => camera.init_transform(*transform),
            IngameCameraKind::FirstPerson(camera) => camera.init_transform(*transform),
            IngameCameraKind::FixedAngle(camera) => camera.init_transform(*transform),
        }
    }
}

pub fn update_transform(
    time: Res<Time>,
    rapier_context: Res<RapierContext>,
    mut camera: Query<(&mut IngameCamera, &mut Transform)>,
) {
    for (mut camera, mut transform) in camera.iter_mut() {
        let actions = camera.actions.clone();
        let dt = time.delta_seconds();
        let new_transform = {
            match &mut camera.kind {
                IngameCameraKind::ThirdPerson(camera) => {
                    camera.update_transform(dt, actions, &rapier_context, *transform)
                }
                IngameCameraKind::FirstPerson(camera) => {
                    camera.update_transform(dt, actions, *transform)
                }
                IngameCameraKind::FixedAngle(camera) => {
                    camera.update_transform(dt, actions, *transform)
                }
            }
        };
        *transform = new_transform;
    }
}

fn reset_actions(mut camera: Query<&mut IngameCamera>) {
    for mut camera in camera.iter_mut() {
        camera.actions = default();
    }
}

fn cursor_grab_system(
    mut windows: ResMut<Windows>,
    actions_frozen: Res<ActionsFrozen>,
) -> Result<()> {
    let window = windows
        .get_primary_mut()
        .context("Failed to get primary window")?;
    if actions_frozen.is_frozen() {
        window.set_cursor_grab_mode(CursorGrabMode::None);
        window.set_cursor_visibility(true);
    } else {
        window.set_cursor_grab_mode(CursorGrabMode::Locked);
        window.set_cursor_visibility(false);
    }
    Ok(())
}
