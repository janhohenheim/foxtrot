use bevy::{prelude::*, render::view::RenderLayers};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_ui_camera);
    app.add_observer(render_ui_to_ui_camera);
}

const UI_RENDER_LAYER: usize = 2;

fn spawn_ui_camera(mut commands: Commands) {
    commands.spawn((
        Name::new("UI Camera"),
        Camera2d,
        // Render all UI to this camera.
        IsDefaultUiCamera,
        Camera {
            // Bump the order to render on top of the view model.
            order: 2,
            ..default()
        },
    ));
}
