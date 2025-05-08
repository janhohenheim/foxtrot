//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{dev_tools::states::log_transitions, prelude::*};

mod debug_ui;
mod input;
mod shader_compilation;
mod validate_preloading;

use crate::screens::{Screen, loading::LoadingScreen};

pub(super) fn plugin(app: &mut App) {
    // Log `Screen` state transitions.
    app.add_systems(
        Update,
        (log_transitions::<Screen>, log_transitions::<LoadingScreen>).chain(),
    );

    app.add_plugins((
        debug_ui::plugin,
        input::plugin,
        validate_preloading::plugin,
        shader_compilation::plugin,
    ));
}
