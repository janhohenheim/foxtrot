//! [Bevy TrenchBroom](https://github.com/Noxmore/bevy_trenchbroom) is the integration layer between Bevy and [TrenchBroom](https://trenchbroom.github.io/).
//! We use TrenchBroom to edit our levels.

use bevy::prelude::*;
use bevy_trenchbroom::prelude::*;
use proxy::RegisterProxies as _;

use crate::{
    gameplay::{npc::Npc, player::Player},
    props::RegisterProps as _,
};
pub(crate) use util::*;

mod proxy;
mod util;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(TrenchBroomPlugin({
        TrenchBroomConfig::new("foxtrot")
            .texture_extensions(to_string_vec(&["png", "jpg", "jpeg"]))
            .texture_exclusions(to_string_vec(&[
                "*_disp_*", "*_arm_*", "*_nor_*", "*_local", "*_normal",
            ]))
            // In Wasm, TrenchBroom classes are not automatically registered.
            // So, we need to manually register the classes here
            .register_props()
            .register_proxies()
            .register_class::<Worldspawn>()
            .register_class::<Npc>()
            .register_class::<Player>();
        #[cfg(not(feature = "native"))]
        let config = config.no_bsp_lighting(true);
        config
    }));
    #[cfg(feature = "native")]
    app.add_systems(Startup, write_trenchbroom_config);
    app.add_plugins((proxy::plugin, util::plugin));
}

fn to_string_vec(slice: &[&str]) -> Vec<String> {
    slice.iter().map(|s| s.to_string()).collect()
}

/// Set up TrenchBroom so that it can create maps for our game.
/// This is intentionally not gated to dev builds so that players can edit the levels themselves if they want.
#[cfg(feature = "native")]
fn write_trenchbroom_config(server: Res<TrenchBroomServer>) {
    info!("Writing TrenchBroom config");
    // Errors at this point usually mean that the player has not installed TrenchBroom.
    // The error messages give more details about the exact issue.
    if let Err(err) = server.config.write_game_config_to_default_directory() {
        warn!("Could not write TrenchBroom game config: {err}");
    }
    if let Err(err) = server.config.add_game_to_preferences_in_default_directory() {
        warn!("Could not add game to TrenchBroom preferences: {err}");
    }
}
