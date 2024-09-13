use bevy::prelude::*;

pub mod action;
pub mod controller;
mod spawn;
mod on_dialog;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        action::plugin,
        controller::plugin,
        spawn::plugin,
        on_dialog::plugin,
    ));
}
