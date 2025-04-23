use bevy::{
    core_pipeline::tonemapping::Tonemapping,
    prelude::*,
    render::{camera::CameraOutputMode, render_resource::BlendState},
};

use crate::CameraOrder;

pub(super) fn plugin(app: &mut App) {
    // HDR is not supported on WebGL2
    #[cfg(not(target_family = "wasm"))]
    app.add_observer(make_hdr_compatible);
}

fn make_hdr_compatible(
    trigger: Trigger<OnAdd, Camera>,
    mut cameras: Query<&mut Camera>,
    mut commands: Commands,
) {
    let mut camera = cameras.get_mut(trigger.entity()).unwrap();
    if camera.order == isize::from(CameraOrder::World) {
        return;
    }
    if camera.hdr {
        // Needed because of https://github.com/bevyengine/bevy/issues/18902
        commands.entity(trigger.entity()).insert(Tonemapping::None);
    }
    // Needed because of https://github.com/bevyengine/bevy/issues/18901
    camera.clear_color = ClearColorConfig::Custom(Color::NONE);
    camera.output_mode = CameraOutputMode::Write {
        blend_state: Some(BlendState::ALPHA_BLENDING),
        clear_color: ClearColorConfig::None,
    };
}
