use crate::player_control::actions::{ActionsFrozen, CameraActions};
use crate::player_control::camera::focus::{set_camera_focus, switch_kind};
use crate::GameState;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use bevy_rapier3d::prelude::*;
pub use first_person::FirstPersonCamera;
use serde::{Deserialize, Serialize};
pub use third_person::ThirdPersonCamera;
use ui::*;

mod first_person;
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
        }
    }

    pub fn up_mut(&mut self) -> &mut Vec3 {
        match &mut self.kind {
            IngameCameraKind::ThirdPerson(camera) => &mut camera.up,
            IngameCameraKind::FirstPerson(camera) => &mut camera.up,
        }
    }

    pub fn forward(&self) -> Vec3 {
        match &self.kind {
            IngameCameraKind::ThirdPerson(camera) => camera.forward(),
            IngameCameraKind::FirstPerson(camera) => camera.forward(),
        }
    }

    pub fn secondary_target_mut(&mut self) -> &mut Option<Vec3> {
        match &mut self.kind {
            IngameCameraKind::ThirdPerson(camera) => &mut camera.secondary_target,
            IngameCameraKind::FirstPerson(camera) => &mut camera.look_target,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Reflect, FromReflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub enum IngameCameraKind {
    ThirdPerson(ThirdPersonCamera),
    FirstPerson(FirstPersonCamera),
}

impl Default for IngameCameraKind {
    fn default() -> Self {
        Self::ThirdPerson(ThirdPersonCamera::default())
    }
}

pub struct CameraPlugin;

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
                    .with_system(cursor_grab_system)
                    .with_system(init_camera)
                    .with_system(
                        switch_kind
                            .label("switch_camera_kind")
                            .before("update_camera_transform"),
                    )
                    .with_system(
                        update_transform
                            .label("update_camera_transform")
                            .after("set_actions")
                            .after("set_camera_focus"),
                    )
                    .with_system(set_camera_focus.label("set_camera_focus"))
                    .with_system(
                        reset_actions
                            .label("reset_movement")
                            .after("update_camera_transform"),
                    ),
            );
    }
}

fn init_camera(mut camera: Query<(&mut IngameCamera, &Transform), Added<IngameCamera>>) {
    for (mut camera, transform) in camera.iter_mut() {
        match &mut camera.kind {
            IngameCameraKind::ThirdPerson(camera) => camera.init_transform(*transform),
            IngameCameraKind::FirstPerson(camera) => camera.init_transform(*transform),
        }
    }
}

fn update_transform(
    time: Res<Time>,
    rapier_context: Res<RapierContext>,
    mut camera: Query<(&mut IngameCamera, &mut Transform)>,
) {
    for (mut camera, mut transform) in camera.iter_mut() {
        let actions = camera.actions.clone();
        let new_transform = {
            match &mut camera.kind {
                IngameCameraKind::ThirdPerson(camera) => camera.update_transform(
                    time.delta_seconds(),
                    actions,
                    &rapier_context,
                    *transform,
                ),
                IngameCameraKind::FirstPerson(camera) => {
                    camera.update_transform(time.delta_seconds(), actions, *transform)
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
