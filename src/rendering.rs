use std::iter;

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
    app.add_systems(Update, make_light_container_unlit);
}

fn make_hdr_compatible(
    trigger: Trigger<OnAdd, Camera>,
    mut cameras: Query<&mut Camera>,
    mut commands: Commands,
) {
    let mut camera = cameras.get_mut(trigger.entity()).unwrap();
    if camera.hdr {
        // Needed because of https://github.com/bevyengine/bevy/issues/18902
        if camera.order != isize::from(CameraOrder::World) {
            commands.entity(trigger.entity()).insert(Tonemapping::None);
        }
        return;
    }
    // Needed because of https://github.com/bevyengine/bevy/issues/18901
    camera.clear_color = ClearColorConfig::Custom(Color::NONE);
    camera.output_mode = CameraOutputMode::Write {
        blend_state: Some(BlendState::ALPHA_BLENDING),
        clear_color: ClearColorConfig::None,
    };
}

#[derive(Component)]
pub(crate) struct ContainsLight;

fn make_light_container_unlit(
    containers: Query<Entity, With<ContainsLight>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    children: Query<&Children>,
    handles: Query<&MeshMaterial3d<StandardMaterial>>,
    mut commands: Commands,
) {
    for container in containers.iter() {
        let mut success = false;
        for child in iter::once(container).chain(children.iter_descendants(container)) {
            let Ok(material) = handles.get(child) else {
                continue;
            };
            let Some(material) = materials.get_mut(material.id()) else {
                warn!("Failed to get a material at runtime. Did you forget to preload it?");
                continue;
            };
            material.unlit = true;
            success = true;
        }
        if success {
            commands.entity(container).remove::<ContainsLight>();
        }
    }
}
