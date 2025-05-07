use bevy::prelude::*;
mod light_window;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(light_window::plugin);
}
