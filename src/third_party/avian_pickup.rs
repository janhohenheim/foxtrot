use avian_pickup::prelude::*;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(AvianPickupPlugin::default());
}
