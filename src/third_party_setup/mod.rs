use bevy::prelude::*;

mod avian;
mod blenvy;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((blenvy::plugin, avian::plugin));
}
