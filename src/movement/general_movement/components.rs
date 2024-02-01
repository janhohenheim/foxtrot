use crate::level_instantiation::spawning::objects::CollisionLayer;
use crate::movement::general_movement::AnimationState;
use bevy::prelude::*;
use bevy_tnua::prelude::*;
use bevy_tnua::TnuaAnimatingState;
use bevy_tnua_xpbd3d::*;
use bevy_xpbd_3d::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Bundle)]
pub(crate) struct CharacterControllerBundle {
    pub(crate) walking: Walking,
    pub(crate) sprinting: Sprinting,
    pub(crate) jumping: Jumping,
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
    pub(crate) fn capsule(height: f32, radius: f32) -> Self {
        Self {
            walking: default(),
            sprinting: default(),
            jumping: default(),
            collider: Collider::capsule(height, radius),
            rigid_body: RigidBody::Dynamic,
            locked_axes: LockedAxes::new().lock_rotation_x().lock_rotation_z(),
            collision_layers: CollisionLayers::new(
                [CollisionLayer::Solid],
                [CollisionLayer::Solid],
            ),
            tnua_sensor_shape: TnuaXpbd3dSensorShape(Collider::capsule(height * 0.9, radius * 0.9)),
            tnua_controller: default(),
            float_height: FloatHeight(height / 2. + 0.001),
            animation_state: default(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Component, Serialize, Deserialize)]
pub(crate) struct Model {
    pub(crate) target: Entity,
}

#[derive(Debug, Clone, PartialEq, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub(crate) struct Walking {
    /// Top speed on the ground
    pub(crate) speed: f32,
    /// Direction in which we want to walk and turn this tick.
    pub(crate) direction: Option<Vec3>,
}

impl Default for Walking {
    fn default() -> Self {
        Self {
            speed: 8.,
            direction: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub(crate) struct Jumping {
    /// The full height of the jump, if the player does not release the button:
    pub(crate) height: f32,
    /// Was jump requested?
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
            multiplier: 1.7,
            requested: false,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
/// Must be larger than the height of the entity's center from the bottom of its
/// collider, or else the character will not float and Tnua will not work properly:
pub(crate) struct FloatHeight(pub(crate) f32);

impl Default for Jumping {
    fn default() -> Self {
        Self {
            height: 3.,
            requested: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Component, Reflect, Default)]
#[reflect(Component)]
pub(crate) struct CharacterAnimations {
    pub(crate) idle: Handle<AnimationClip>,
    pub(crate) walk: Handle<AnimationClip>,
    pub(crate) aerial: Handle<AnimationClip>,
}
