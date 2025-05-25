//! [Avian](https://github.com/Jondolf/avian) is our physics engine.

use avian3d::prelude::*;
use bevy::prelude::*;
#[cfg(feature = "hot_patch")]
use bevy_simple_subsecond_system::hot;

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

#[cfg_attr(feature = "hot_patch", hot)]
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
