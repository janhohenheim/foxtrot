use bevy::{
    asset::RenderAssetUsages,
    image::{ImageAddressMode, ImageSamplerDescriptor},
    prelude::*,
};
#[cfg(feature = "hot_patch")]
use bevy_simple_subsecond_system::hot;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, move_textures_to_render_world);
}

pub(crate) fn default_image_sampler_descriptor() -> ImageSamplerDescriptor {
    ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        address_mode_w: ImageAddressMode::Repeat,
        anisotropy_clamp: 16,
        ..ImageSamplerDescriptor::linear()
    }
}

#[cfg_attr(feature = "hot_patch", hot)]
fn move_textures_to_render_world(
    mut events: EventReader<AssetEvent<Image>>,
    mut images: ResMut<Assets<Image>>,
    assets: Res<AssetServer>,
) {
    for event in events.read() {
        let AssetEvent::LoadedWithDependencies { id } = event else {
            continue;
        };

        let Some(path) = assets.get_path(id.untyped()) else {
            continue;
        };

        let path = path.to_string();

        const PATHS_WITH_MESH_TEXTURES: &[&str] = &["textures/", "models/"];
        if !PATHS_WITH_MESH_TEXTURES.iter().any(|p| path.starts_with(p)) {
            // Textures outside these paths are e.g. part of the UI, which would stop rendering
            // if we set it to `RENDER_WORLD`. It also wouldn't make sense to change the sampler
            // for those, as we look at them with a pixel-perfect camera from a single angle.
            continue;
        }

        let image = images.get_mut(*id).unwrap();
        image.asset_usage = RenderAssetUsages::RENDER_WORLD;
    }
}
