use crate::movement::{character_controller::AnimationState, physics::CollisionLayer};
use bevy::prelude::*;
use bevy_tnua::{prelude::*, TnuaAnimatingState};
use bevy_tnua_xpbd3d::*;
use bevy_xpbd_3d::prelude::*;
use serde::{Deserialize, Serialize};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Jump>().register_type::<Walk>();
}

#[derive(Bundle)]
pub(crate) struct CharacterControllerBundle {
    pub(crate) walking: Walk,
    pub(crate) sprinting: Sprinting,
    pub(crate) jumping: Jump,
    pub(crate) collider: Collider,
    pub(crate) rigid_body: RigidBody,
    pub(crate) locked_axes: LockedAxes,
    pub(crate) collision_layers: CollisionLayers,
    pub(crate) tnua_sensor_shape: TnuaXpbd3dSensorShape,
    pub(crate) tnua_controller: TnuaControllerBundle,
    pub(crate) float_height: FloatHeight,
    pub(crate) animation_state: TnuaAnimatingState<AnimationState>,
}

impl CharacterControllerBundle {
    pub(crate) fn capsule(height: f32, radius: f32, scale_y: f32) -> Self {
        Self {
            walking: default(),
            sprinting: default(),
            jumping: default(),
            collider: Collider::capsule(height, radius),
            rigid_body: RigidBody::Dynamic,
            locked_axes: LockedAxes::new().lock_rotation_x().lock_rotation_z(),
            collision_layers: CollisionLayers::new(
                [CollisionLayer::Character],
                [
                    CollisionLayer::Player,
                    CollisionLayer::Character,
                    CollisionLayer::Terrain,
                    CollisionLayer::Sensor,
                ],
            ),
            tnua_sensor_shape: TnuaXpbd3dSensorShape(Collider::capsule(
                height * 0.95,
                radius * 0.95,
            )),
            tnua_controller: default(),
            float_height: FloatHeight((height / 2. + radius) * scale_y),
            animation_state: default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub(crate) struct Walk {
    /// Top speed on the ground
    pub(crate) speed: f32,
    /// Direction in which we want to walk and turn this tick.
    pub(crate) direction: Option<Vec3>,
}

impl Default for Walk {
    fn default() -> Self {
        Self {
            speed: 8.,
            direction: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub(crate) struct Jump {
    /// The full height of the jump, if the player does not release the button
    pub(crate) height: f32,
    /// Was jump requested this frame?
    pub(crate) requested: bool,
}

#[derive(Debug, Clone, PartialEq, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub(crate) struct Sprinting {
    /// The speed multiplier when sprinting
    pub(crate) multiplier: f32,
    /// Was sprinting requested?
    pub(crate) requested: bool,
}

impl Default for Sprinting {
    fn default() -> Self {
        Self {
            multiplier: 1.5,
            requested: false,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
/// Must be larger than the height of the entity's center from the bottom of its
/// collider, or else the character will not float and Tnua will not work properly
pub(crate) struct FloatHeight(pub(crate) f32);

impl Default for Jump {
    fn default() -> Self {
        Self {
            height: 1.0,
            requested: false,
        }
    }
}
