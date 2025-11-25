//! [Avian](https://github.com/Jondolf/avian) is our physics engine.

use avian3d::prelude::*;
use bevy::{ecs::entity_disabling::Disabled, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(PhysicsPlugins::default())
        .add_observer(enable_interpolation);
}

#[derive(Debug, PhysicsLayer, Default)]
pub(crate) enum CollisionLayer {
    #[default]
    Default,
    Prop,
    Character,
}

fn enable_interpolation(
    add: On<Add, RigidBody>,
    bodies: Query<&RigidBody, Allow<Disabled>>,
    mut commands: Commands,
) {
    if bodies.get(add.entity).is_ok_and(|b| !b.is_static()) {
        commands.entity(add.entity).insert(TransformInterpolation);
    }
}
