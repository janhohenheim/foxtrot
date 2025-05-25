//! The UI camera is a 2D camera that renders all UI elements in front of everything else.
//! We use a dedicated camera for this because our other two cameras, namely the world and view model cameras,
//! don't exist during non-gameplay screens such as the main menu.

use bevy::prelude::*;
#[cfg(feature = "hot_patch")]
use bevy_simple_subsecond_system::hot;

use crate::CameraOrder;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_ui_camera);
    app.register_type::<UiCamera>();
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub(crate) struct UiCamera;

#[cfg_attr(feature = "hot_patch", hot)]
fn spawn_ui_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("UI Camera"),
        UiCamera,
        Camera2d,
        // Render all UI to this camera.
        IsDefaultUiCamera,
        Camera {
            // The UI camera order is the highest.
            order: CameraOrder::Ui.into(),
            ..default()
        },
    ));
}
