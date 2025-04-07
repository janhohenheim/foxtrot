use bevy::prelude::*;

pub(crate) mod avian3d;
pub(crate) mod bevy_enhanced_input;
pub(crate) mod bevy_tnua;
pub(crate) mod bevy_trenchbroom;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        bevy_trenchbroom::plugin,
        avian3d::plugin,
        bevy_enhanced_input::plugin,
        bevy_tnua::plugin,
    ));
}
