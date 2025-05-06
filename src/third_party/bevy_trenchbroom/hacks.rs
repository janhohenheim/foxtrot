use std::path::Path;

use bevy::{image::ImageLoaderSettings, prelude::*};
use regex::Regex;

use crate::default_image_sampler_descriptor;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, preload_base_color_textures);
}

#[derive(Resource)]
#[allow(dead_code)]
struct PreloadedBaseColorTextures(Vec<Handle<Image>>);

/// Hack BTB loads all textures as sRGB, which is wrong for everything except base color textures.
/// So our fork load all textures as linear. That in turn is wrong for the base color textures,
/// so we preload them using the correct sampler.
fn preload_base_color_textures(asset_server: Res<AssetServer>, mut commands: Commands) {
    // hack: read the map from the file system
    let map = include_str!("../../../assets/maps/foxtrot/foxtrot.map");
    let texture_regex = Regex::new(r"\) ([\w\d\/_]+) \[").unwrap();
    let mut handles = Vec::new();
    for cap in texture_regex.captures_iter(map) {
        let texture_path = cap[1].to_string();
        let texture_path = Path::new(&texture_path);
        let ext = if cfg!(feature = "dev") { "png" } else { "ktx2" };
        let prefix = Path::new("textures/");
        let texture_path = prefix.join(texture_path).with_extension(ext);
        handles.push(asset_server.load_with_settings(
            texture_path,
            |settings: &mut ImageLoaderSettings| {
                *settings.sampler.get_or_init_descriptor() = default_image_sampler_descriptor();
                settings.is_srgb = true;
            },
        ));
    }
    commands.insert_resource(PreloadedBaseColorTextures(handles));
}
