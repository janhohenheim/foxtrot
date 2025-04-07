//! Spawn the main level.

use bevy::prelude::*;

use crate::screens::Screen;

pub(super) fn plugin(_app: &mut App) {
    // No setup required for this plugin.
    // It's still good to have a function here so that we can add some setup
    // later if needed.
}

/// A [`Command`] to spawn the level.
/// Functions that accept only `&mut World` as their parameter implement [`Command`].
/// We use this style when a command requires no configuration.
pub(crate) fn spawn_level(world: &mut World) {
    // The only thing we have in our level is a player,
    // but add things like walls etc. here.
    let asset_server = world.resource::<AssetServer>();
    world.spawn((
        SceneRoot(
            //  Run ./scripts/compile_maps.sh and change .map to .bsp when you're done prototyping and want some extra performance
            asset_server.load("maps/foxtrot/foxtrot.map#Scene"),
        ),
        StateScoped(Screen::Gameplay),
    ));
}
