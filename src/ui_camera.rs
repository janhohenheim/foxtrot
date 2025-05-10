//! The UI camera is a 2D camera that renders all UI elements in front of everything else.
//! We use a dedicated camera for this because our other two cameras, namely the world and view model cameras,
//! don't exist during non-gameplay screens such as the main menu.

use bevy::{prelude::*, render::view::RenderLayers};

use crate::{CameraOrder, RenderLayer};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_ui_camera);
    app.register_type::<UiCamera>();
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub(crate) struct UiCamera;

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
        // This line causes https://github.com/bevyengine/bevy/issues/19166 and https://github.com/bevyengine/bevy/issues/19167
        RenderLayers::from(RenderLayer::UI),
    ));
}
