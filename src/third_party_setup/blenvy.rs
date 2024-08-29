use bevy::prelude::*;
use blenvy::BlenvyPlugin;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(BlenvyPlugin::default());
}
