//! [Bevy TrenchBroom](https://github.com/Noxmore/bevy_trenchbroom) is the integration layer between Bevy and [TrenchBroom](https://trenchbroom.github.io/).
//! We use TrenchBroom to edit our levels.

use bevy::{image::ImageSampler, prelude::*};
use bevy_trenchbroom::prelude::*;

pub(crate) use util::*;

use crate::asset_processing::default_image_sampler_descriptor;

mod hacks;
mod proxy;
mod util;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(TrenchBroomPlugin({
        TrenchBroomConfig::new("foxtrot")
            .texture_extensions(to_string_vec(&["png", "jpg", "jpeg"]))
            .texture_exclusions(to_string_vec(&[
                "*_disp_*", "*_arm_*", "*_nor_*", "*_local", "*_normal",
            ]))
            // We only use BSPs for light maps.
            .no_bsp_lighting(true)
            .texture_sampler(texture_sampler())
            .linear_filtering()
    }));
    #[cfg(feature = "native")]
    app.add_systems(Startup, write_trenchbroom_config);
    app.add_plugins((proxy::plugin, util::plugin, hacks::plugin));
    app.register_type::<Worldspawn>();
}

pub(crate) fn texture_sampler() -> ImageSampler {
    let mut sampler = ImageSampler::linear();
    *sampler.get_or_init_descriptor() = default_image_sampler_descriptor();
    sampler
}

fn to_string_vec(slice: &[&str]) -> Vec<String> {
    slice.iter().map(|s| s.to_string()).collect()
}

/// Set up TrenchBroom so that it can create maps for our game.
/// This is intentionally not gated to dev builds so that players can edit the levels themselves if they want.
#[cfg(feature = "native")]
fn write_trenchbroom_config(server: Res<TrenchBroomServer>, type_registry: Res<AppTypeRegistry>) {
    info!("Writing TrenchBroom config");
    // Errors at this point usually mean that the player has not installed TrenchBroom.
    // The error messages give more details about the exact issue.
    if let Err(err) = server
        .config
        .write_game_config_to_default_directory(&type_registry.read())
    {
        warn!("Could not write TrenchBroom game config: {err}");
    }
    if let Err(err) = server.config.add_game_to_preferences_in_default_directory() {
        warn!("Could not add game to TrenchBroom preferences: {err}");
    }
}
