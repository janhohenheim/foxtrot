use bevy::prelude::*;

use crate::CameraOrder;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_ui_camera);
}

fn spawn_ui_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("UI Camera"),
        Camera2d,
        // Render all UI to this camera.
        IsDefaultUiCamera,
        Camera {
            // Bump the order to render on top of the view model.
            order: CameraOrder::Ui.into(),
            ..default()
        },
    ));
}
