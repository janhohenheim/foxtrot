//! Plugin handling the player character in particular.
//! Note that this is separate from the `movement` module as that could be used
//! for other characters as well.

use bevy::prelude::*;

pub mod camera;
pub mod initialize;
pub mod input;
mod interactions;
mod on_dialog;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>();

    app.add_plugins((
        camera::plugin,
        input::plugin,
        initialize::plugin,
        on_dialog::plugin,
        interactions::plugin,
    ));
}

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
#[reflect(Component)]
pub struct Player;
