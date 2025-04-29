//! The game's main screen states and transitions between them.

mod credits;
mod gameplay;
mod loading;
mod spawn_level;
mod splash;
mod title;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();
    app.enable_state_scoped_entities::<Screen>();

    app.add_plugins((
        credits::plugin,
        gameplay::plugin,
        spawn_level::plugin,
        loading::plugin,
        splash::plugin,
        title::plugin,
    ));
}

/// The game's main screen states.
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
pub(crate) enum Screen {
    #[default]
    Splash,
    Title,
    Credits,
    Settings,
    Loading,
    SpawnLevel,
    Gameplay,
}
