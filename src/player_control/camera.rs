use crate::file_system_interaction::asset_loading::ConfigAssets;
use crate::file_system_interaction::config::GameConfig;
use crate::level_instantiation::spawning::objects::skydome::Skydome;
use crate::player_control::actions::{ActionsFrozen, CameraAction};
use crate::player_control::camera::focus::{set_camera_focus, switch_kind};
use crate::util::log_error::log_errors;
use crate::GameState;
use anyhow::{Context, Result};
use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use bevy_rapier3d::prelude::*;
pub use first_person::FirstPersonCamera;
pub use fixed_angle::FixedAngleCamera;
use leafwing_input_manager::prelude::ActionState;
use serde::{Deserialize, Serialize};
pub use third_person::ThirdPersonCamera;
use ui::*;

mod first_person;
mod fixed_angle;
pub mod focus;
mod third_person;
mod ui;
mod util;

#[derive(
    Debug, Clone, PartialEq, Component, Reflect, Serialize, Deserialize, FromReflect, Default,
)]
#[reflect(Component, Serialize, Deserialize)]
pub struct IngameCamera {
    pub kind: IngameCameraKind,
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
            .register_type::<FirstPersonCamera>()
            .register_type::<FixedAngleCamera>()
            .init_resource::<ForceCursorGrabMode>()
            .add_startup_system(spawn_ui_camera)
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(despawn_ui_camera))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(cursor_grab_system.pipe(log_errors))
                    .with_system(init_camera.pipe(log_errors))
                    .with_system(set_camera_focus.pipe(log_errors).label(SetCameraFocusLabel))
                    .with_system(switch_kind.after(SetCameraFocusLabel))
                    .with_system(
                        update_transform
                            .pipe(log_errors)
                            .label(UpdateCameraTransformLabel)
                            .after(switch_kind),
                    )
                    .with_system(update_config.pipe(log_errors))
                    .with_system(move_skydome.after(UpdateCameraTransformLabel)),
            );
    }
}

#[derive(SystemLabel)]
pub struct UpdateCameraTransformLabel;

fn init_camera(
    mut camera: Query<(&Transform, &mut IngameCamera), Added<IngameCamera>>,
    config_handles: Res<ConfigAssets>,
    config: Res<Assets<GameConfig>>,
) -> Result<()> {
    #[cfg(feature = "tracing")]
    let _span = info_span!("init_camera").entered();
    for (transform, mut camera) in camera.iter_mut() {
        let game_config = config
            .get(&config_handles.game)
            .context("Failed to get game config from handle")?;
        match &mut camera.kind {
            IngameCameraKind::ThirdPerson(camera) => {
                camera.transform = *transform;
                camera.config = game_config.clone();
            }
            IngameCameraKind::FirstPerson(camera) => {
                camera.transform = *transform;
                camera.config = game_config.clone();
            }
            IngameCameraKind::FixedAngle(camera) => {
                camera.transform = *transform;
                camera.config = game_config.clone();
            }
        }
    }
    Ok(())
}

pub fn update_transform(
    time: Res<Time>,
    rapier_context: Res<RapierContext>,
    mut camera: Query<(
        &ActionState<CameraAction>,
        &mut IngameCamera,
        &mut Transform,
    )>,
) -> Result<()> {
    #[cfg(feature = "tracing")]
    let _span = info_span!("update_transform").entered();
    for (actions, mut camera, mut transform) in camera.iter_mut() {
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
        }?;
        *transform = new_transform;
    }
    Ok(())
}

fn update_config(
    config: Res<Assets<GameConfig>>,
    mut config_asset_events: EventReader<AssetEvent<GameConfig>>,
    mut camera_query: Query<&mut IngameCamera>,
) -> Result<()> {
    #[cfg(feature = "tracing")]
    let _span = info_span!("update_config").entered();
    for event in config_asset_events.iter() {
        match event {
            AssetEvent::Created { handle } | AssetEvent::Modified { handle } => {
                // Guaranteed by Bevy to not fail
                let config = config
                    .get(handle)
                    .context("Failed to get config even though it was just created")?;
                for mut camera in camera_query.iter_mut() {
                    *match camera.kind {
                        IngameCameraKind::ThirdPerson(ref mut camera) => &mut camera.config,
                        IngameCameraKind::FirstPerson(ref mut camera) => &mut camera.config,
                        IngameCameraKind::FixedAngle(ref mut camera) => &mut camera.config,
                    } = config.clone();
                }
            }
            AssetEvent::Removed { .. } => {}
        }
    }
    Ok(())
}

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
    mut windows: ResMut<Windows>,
    actions_frozen: Res<ActionsFrozen>,
    force_cursor_grab: Res<ForceCursorGrabMode>,
) -> Result<()> {
    #[cfg(feature = "tracing")]
    let _span = info_span!("cursor_grab_system").entered();
    let window = windows
        .get_primary_mut()
        .context("Failed to get primary window")?;
    if let Some(mode) = force_cursor_grab.0 {
        window.set_cursor_grab_mode(mode);
        window.set_cursor_visibility(mode != CursorGrabMode::Locked);
        return Ok(());
    }
    if actions_frozen.is_frozen() {
        window.set_cursor_grab_mode(CursorGrabMode::None);
        window.set_cursor_visibility(true);
    } else {
        window.set_cursor_grab_mode(CursorGrabMode::Locked);
        window.set_cursor_visibility(false);
    }
    Ok(())
}
