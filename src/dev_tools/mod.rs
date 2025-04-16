//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{dev_tools::states::log_transitions, prelude::*};

mod debug_ui;
mod input;
mod validate_preloading;

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    // Log `Screen` state transitions.
    app.add_systems(Update, log_transitions::<Screen>);

    app.add_plugins((input::plugin, debug_ui::plugin, validate_preloading::plugin));
}
