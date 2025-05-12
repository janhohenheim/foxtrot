//! [Avian](https://github.com/Jondolf/avian) is our physics engine.

use avian3d::prelude::*;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(PhysicsPlugins::default());
    app.add_observer(enable_interpolation);
}

#[derive(Debug, PhysicsLayer, Default)]
pub(crate) enum CollisionLayer {
    #[default]
    Default,
    Prop,
    Character,
}

fn enable_interpolation(
    trigger: Trigger<OnAdd, RigidBody>,
    rigid_body: Query<&RigidBody>,
    mut commands: Commands,
) {
    let Ok(rigid_body) = rigid_body.get(trigger.target()) else {
        return;
    };
    if rigid_body.is_dynamic() {
        commands
            .entity(trigger.target())
            .insert(TransformInterpolation);
    }
}
