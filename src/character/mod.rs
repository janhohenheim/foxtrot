use bevy::prelude::*;

pub mod action;
pub mod controller;
mod spawn;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((action::plugin, controller::plugin, spawn::plugin));
}

