use avian3d::prelude::ColliderConstructorHierarchy;
use bevy::{
    asset::RenderAssetUsages,
    image::{ImageFilterMode, ImageSampler, ImageSamplerDescriptor},
    prelude::*,
};
use bevy_trenchbroom::physics::SceneCollidersReady;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, configure_textures);
    app.add_observer(configure_gltfs_after_collider_constructors);
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

fn configure_gltfs_after_collider_constructors(
    trigger: Trigger<OnRemove, ColliderConstructorHierarchy>,
    children: Query<&Children>,
    mesh_handles: Query<&Mesh3d>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for child in children.iter_descendants(trigger.target()) {
        let Ok(mesh) = mesh_handles.get(child) else {
            continue;
        };

        let Some(mesh) = meshes.get_mut(mesh) else {
            continue;
        };

        mesh.asset_usage = RenderAssetUsages::RENDER_WORLD;
    }
}

fn configure_gltfs_after_trenchbroom(
    trigger: Trigger<SceneCollidersReady>,
    children: Query<&Children>,
    mesh_handles: Query<&Mesh3d>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for child in children.iter_descendants(trigger.target()) {
        let Ok(mesh) = mesh_handles.get(child) else {
            continue;
        };

        let Some(mesh) = meshes.get_mut(mesh) else {
            continue;
        };

        mesh.asset_usage = RenderAssetUsages::RENDER_WORLD;
    }
}
