//! There are a couple of Bevy issues when using HDR with a multi-camera setup,
//! so this module contains some workarounds.

use bevy::prelude::*;
#[cfg(feature = "hot_patch")]
use bevy_simple_subsecond_system::hot;

use bevy::{
    core_pipeline::tonemapping::Tonemapping,
    render::{camera::CameraOutputMode, render_resource::BlendState},
};

use crate::CameraOrder;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(make_hdr_compatible);
}

#[cfg_attr(feature = "hot_patch", hot)]
fn make_hdr_compatible(
    trigger: Trigger<OnAdd, Camera>,
    mut cameras: Query<&mut Camera>,
    mut commands: Commands,
) {
    let entity = trigger.target();
    let mut camera = cameras.get_mut(entity).unwrap();
    if camera.order == isize::from(CameraOrder::World) {
        // Use the world model camera to determine tonemapping.
        return;
    }
    // Needed because of https://github.com/bevyengine/bevy/issues/18902
    commands.entity(entity).insert(Tonemapping::None);
    // Needed because of https://github.com/bevyengine/bevy/issues/18901
    // and https://github.com/bevyengine/bevy/issues/18903
    camera.clear_color = ClearColorConfig::Custom(Color::NONE);
    camera.output_mode = CameraOutputMode::Write {
        blend_state: Some(BlendState::ALPHA_BLENDING),
        clear_color: ClearColorConfig::None,
    };
}
