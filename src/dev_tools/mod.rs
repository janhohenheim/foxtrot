//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{dev_tools::states::log_transitions, prelude::*};
use bevy_simple_subsecond_system::prelude::*;

mod debug_ui;
mod input;
mod validate_preloading;

use crate::screens::{Screen, loading::LoadingScreen};

pub(super) fn plugin(app: &mut App) {
    // Log `Screen` state transitions.
    app.add_systems(
        Update,
        (log_transitions::<Screen>, log_transitions::<LoadingScreen>).chain(),
    );

    app.add_plugins((
        SimpleSubsecondPlugin::default(),
        debug_ui::plugin,
        input::plugin,
        validate_preloading::plugin,
    ));

    app.add_systems(Update, debug_print);
}

#[hot]
fn debug_print() {
    println!("Debug");
}
