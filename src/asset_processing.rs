use bevy::{
    asset::RenderAssetUsages,
    image::{ImageFilterMode, ImageSampler, ImageSamplerDescriptor},
    prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, configure_textures);
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
