use bevy::prelude::*;

mod avian;
mod blenvy;
mod lwim;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((blenvy::plugin, avian::plugin, lwim::plugin));
}
