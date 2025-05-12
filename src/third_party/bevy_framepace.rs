//! [bevy_framepace](https://github.com/aevyrie/bevy_framepace) reduces the latency between
//! inputs and game events.

use bevy::prelude::*;
use bevy_framepace::FramepacePlugin;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(FramepacePlugin);
}
