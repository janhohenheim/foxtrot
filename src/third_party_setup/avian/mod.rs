use avian3d::prelude::*;
use bevy::prelude::*;

mod collision_layer;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((PhysicsPlugins::default(), collision_layer::plugin));
}
