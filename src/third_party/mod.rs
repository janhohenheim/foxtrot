//! Third-party plugins.
//!
//! We use one file per plugin to encapsulate setup or boilerplate necessary for that plugin.
//! Many plugins don't require any setup, but it's still nice to have them in an own file so
//! that we are ready to add convenience methods or similar when needed.

use bevy::prelude::*;

pub(crate) mod avian3d;
mod avian_pickup;
mod bevy_enhanced_input;
mod bevy_framepace;
#[cfg(feature = "native")]
mod bevy_hanabi;
pub(crate) mod bevy_landmass;
mod bevy_tnua;
pub(crate) mod bevy_trenchbroom;
pub(crate) mod bevy_yarnspinner;
mod fixes;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        fixes::plugin,
        bevy_trenchbroom::plugin,
        avian3d::plugin,
        bevy_enhanced_input::plugin,
        bevy_tnua::plugin,
        bevy_landmass::plugin,
        bevy_yarnspinner::plugin,
        avian_pickup::plugin,
        #[cfg(feature = "native")]
        bevy_hanabi::plugin,
        bevy_framepace::plugin,
    ));
}
