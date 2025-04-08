use bevy::prelude::*;

mod avian3d;
mod bevy_enhanced_input;
mod bevy_landmass;
mod bevy_tnua;
pub(crate) mod bevy_trenchbroom;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        bevy_trenchbroom::plugin,
        avian3d::plugin,
        bevy_enhanced_input::plugin,
        bevy_tnua::plugin,
        bevy_landmass::plugin,
    ));
}
