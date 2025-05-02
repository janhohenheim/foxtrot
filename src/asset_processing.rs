use bevy::{
    asset::RenderAssetUsages,
    image::{ImageFilterMode, ImageSampler, ImageSamplerDescriptor},
    prelude::*,
};
use bevy_trenchbroom::physics::SceneCollidersReady;

use crate::third_party::bevy_trenchbroom::Worldspawn;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, configure_textures);
    app.add_observer(configure_gltfs_after_trenchbroom);
}

fn configure_textures(
    mut events: EventReader<AssetEvent<Image>>,
    mut images: ResMut<Assets<Image>>,
) {
    for event in events.read() {
        let AssetEvent::LoadedWithDependencies { id } = event else {
            continue;
        };

        let image = images.get_mut(*id).unwrap();
        image.asset_usage = RenderAssetUsages::RENDER_WORLD;
        if matches!(image.sampler, ImageSampler::Default) {
            image.sampler = ImageSampler::Descriptor(ImageSamplerDescriptor::default());
        }
        let ImageSampler::Descriptor(desc) = &mut image.sampler else {
            unreachable!();
        };

        // Enable trilinear filtering. This will allow us to use mipmaps.
        desc.min_filter = ImageFilterMode::Linear;
        desc.mipmap_filter = ImageFilterMode::Linear;
        desc.mag_filter = ImageFilterMode::Linear;

        // Enable anisotropic filtering. This will make the texture look better at an angle.
        desc.anisotropy_clamp = 16;
    }
}

fn configure_gltfs_after_trenchbroom(
    trigger: Trigger<SceneCollidersReady>,
    world_spawn: Query<(), With<Worldspawn>>,
    children: Query<&Children>,
    mesh_handles: Query<&Mesh3d>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let world_spawn = children
        .iter_descendants(trigger.target())
        .find(|child| world_spawn.contains(*child));
    let Some(world_spawn) = world_spawn else {
        warn!("Level has no world spawn");
        return;
    };

    for child in children.iter_descendants(world_spawn) {
        let Ok(mesh) = mesh_handles.get(child) else {
            continue;
        };

        let Some(mesh) = meshes.get_mut(mesh) else {
            continue;
        };

        mesh.asset_usage = RenderAssetUsages::RENDER_WORLD;
    }
}
