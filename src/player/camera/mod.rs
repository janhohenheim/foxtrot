use crate::ui_camera::UiCamera;
use bevy::prelude::*;
use control::CameraAction;
use leafwing_input_manager::prelude::*;

mod control;
mod on_dialog;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<(PlayerCamera, PlayerCameraConfig)>();
    app.add_plugins((on_dialog::plugin, control::plugin));
}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct PlayerCamera {
    pub follow: Transform,
    pub offset: Transform,
}

impl PlayerCamera {
    pub fn transform(self) -> Transform {
        self.follow * self.offset
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PlayerCameraConfig {
    pub sensitivity: Vec2,
}

impl Default for PlayerCameraConfig {
    fn default() -> Self {
        Self {
            sensitivity: Vec2::new(0.001, 0.001),
        }
    }
}

pub fn spawn_player_camera(world: &mut World) {
    let ui_cameras: Vec<_> = world
        .query_filtered::<Entity, With<UiCamera>>()
        .iter(&world)
        .collect();
    for ui_camera in ui_cameras {
        world.entity_mut(ui_camera).despawn_recursive();
    }
    world.spawn((
        Name::new("Player Camera"),
        Camera3dBundle::default(),
        IsDefaultUiCamera,
        PlayerCamera::default(),
        PlayerCameraConfig::default(),
        InputManagerBundle::with_map(CameraAction::default_input_map()),
    ));
}
