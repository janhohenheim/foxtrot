use crate::ui_camera::UiCamera;
use bevy::prelude::*;

mod first_person;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<PlayerCamera>();
    app.add_plugins((first_person::plugin,));
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PlayerCamera {
    pub sensitivity: Vec2,
}

impl Default for PlayerCamera {
    fn default() -> Self {
        Self {
            sensitivity: Vec2::new(0.001, 0.001),
        }
    }
}

pub fn switch_to_first_person_camera(world: &mut World) {
    let other_cameras = world
        .query_filtered::<Entity, Or<(With<PlayerCamera>, With<UiCamera>)>>()
        .iter(&world)
        .collect::<Vec<_>>();
    for camera in other_cameras {
        world.entity_mut(camera).despawn_recursive();
    }
    world.spawn(first_person::first_person_camera_bundle());
}
