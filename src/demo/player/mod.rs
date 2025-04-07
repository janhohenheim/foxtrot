//! Plugin handling the player character in particular.
//! Note that this is separate from the `movement` module as that could be used
//! for other characters as well.

use avian3d::prelude::*;
use bevy::{
    ecs::{component::ComponentId, world::DeferredWorld},
    prelude::*,
};
use bevy_enhanced_input::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua_avian3d::TnuaAvian3dSensorShape;
use bevy_trenchbroom::prelude::*;

use crate::third_party::bevy_trenchbroom::LoadTrenchbroomModel;

use super::movement::MovementController;

pub(crate) mod assets;
pub(crate) mod camera;
pub(crate) mod input;
pub(crate) mod movement;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>();
    app.add_plugins((
        assets::plugin,
        input::plugin,
        movement::plugin,
        camera::plugin,
    ));
}

#[derive(
    PointClass, Component, ActionsMarker, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect,
)]
#[reflect(Component)]
#[require(Transform, Visibility)]
#[model("models/suzanne/Suzanne.gltf")]
#[component(on_add = Self::on_add)]
pub(crate) struct Player;

impl Player {
    fn on_add(mut world: DeferredWorld, entity: Entity, _id: ComponentId) {
        let Some(asset_server) = world.get_resource::<AssetServer>() else {
            return;
        };

        world.commands().entity(entity).insert((
            RigidBody::Dynamic,
            TrenchBroomGltfRotationFix,
            Actions::<Player>::default(),
            MovementController::default(),
            // The player character needs to be configured as a dynamic rigid body of the physics
            // engine.
            Collider::capsule(0.5, 1.0),
            // This is Tnua's interface component.
            TnuaController::default(),
            // A sensor shape is not strictly necessary, but without it we'll get weird results.
            TnuaAvian3dSensorShape(Collider::cylinder(0.49, 0.0)),
            // Tnua can fix the rotation, but the character will still get rotated before it can do so.
            // By locking the rotation we can prevent this.
            LockedAxes::ROTATION_LOCKED,
        ));
    }
}
