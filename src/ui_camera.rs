use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<UiCamera>();
}

#[derive(Debug, Component, Clone, Copy, Reflect)]
#[reflect(Debug, Component)]
pub struct UiCamera;

pub fn spawn_ui_camera(world: &mut World) {
    world.spawn((
        Name::new("UI Camera"),
        Camera2dBundle::default(),
        // Render all UI to this camera.
        IsDefaultUiCamera,
        UiCamera,
    ));
}
