//! The game's main screen states and transitions between them.

mod credits;
mod gameplay;
pub(crate) mod loading;
mod settings;
mod splash;
mod title;

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_state::<Screen>();

    app.add_plugins((
        credits::plugin,
        gameplay::plugin,
        loading::plugin,
        settings::plugin,
        splash::plugin,
        title::plugin,
    ));
}

/// The game's main screen states.
#[derive(States, Debug, Hash, PartialEq, Eq, Clone, Default)]
#[states(scoped_entities)]
pub(crate) enum Screen {
    #[default]
    Splash,
    Title,
    Credits,
    Settings,
    Loading,
    Gameplay,
}
