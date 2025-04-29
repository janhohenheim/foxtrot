//! [Bevy TrenchBroom](https://github.com/Noxmore/bevy_trenchbroom) is the integration layer between Bevy and [TrenchBroom](https://trenchbroom.github.io/).
//! We use TrenchBroom to edit our levels.

use std::borrow::Cow;

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
        let config = TrenchBroomConfig::new("foxtrot")
            .generic_material_extension("material.toml")
            .texture_exclusions(
                ["*_disp_*", "*_arm_*", "*_nor_*"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<_>>(),
            )
            // In Wasm, TrenchBroom classes are not automatically registered.
            // So, we need to manually register the classes here
            .register_props()
            .register_proxies()
            .register_class::<Worldspawn>()
            .register_class::<Npc>()
            .register_class::<Player>();
        #[cfg(target_arch = "wasm32")]
        let config = config.no_bsp_lighting(true);
        config
    }));
    app.add_systems(Startup, write_trenchbroom_config);
    app.add_plugins((proxy::plugin, util::plugin));
}

/// Set up TrenchBroom so that it can create maps for our game.
/// This is intentionally not gated to dev builds so that players can edit the levels themselves if they want.
fn write_trenchbroom_config(server: Res<TrenchBroomServer>) {
    #[cfg(target_arch = "wasm32")]
    let _ = server;
    #[cfg(not(target_arch = "wasm32"))]
    {
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
}
