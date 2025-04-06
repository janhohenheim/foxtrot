use bevy::prelude::*;

pub mod avian3d;
pub mod bevy_trenchbroom;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((bevy_trenchbroom::plugin, avian3d::plugin));
}
