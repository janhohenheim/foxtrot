//! [Avian Pickup](https://github.com/janhohenheim/avian_pickup) implements the feature where we press left click
//! to pick up and throw objects.

use avian_pickup::prelude::*;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(AvianPickupPlugin::default());
}
