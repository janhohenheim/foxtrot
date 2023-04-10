use crate::level_instantiation::spawning::objects::skydome::Skydome;
use crate::player_control::camera::IngameCamera;
use bevy::prelude::*;

pub(crate) fn move_skydome(
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
