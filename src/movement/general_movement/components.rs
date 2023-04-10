use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Bundle)]
pub(crate) struct CharacterControllerBundle {
    pub(crate) gravity_scale: GravityScale,
    pub(crate) mass: ColliderMassProperties,
    pub(crate) read_mass: ReadMassProperties,
    pub(crate) walking: Walking,
    pub(crate) jumping: Jumping,
    pub(crate) grounded: Grounded,
    pub(crate) damping: Damping,
    pub(crate) rigid_body: RigidBody,
    pub(crate) locked_axes: LockedAxes,
    pub(crate) collider: Collider,
    pub(crate) force: ExternalForce,
    pub(crate) impulse: ExternalImpulse,
    pub(crate) velocity: Velocity,
    pub(crate) dominance: Dominance,
}

impl Default for CharacterControllerBundle {
    fn default() -> Self {
        Self {
            read_mass: default(),
            gravity_scale: GravityScale(1.0),
            force: default(),
            mass: ColliderMassProperties::Mass(3.0),
            walking: default(),
            jumping: default(),
            grounded: default(),
            damping: Damping {
                linear_damping: 1.5,
                ..default()
            },
            collider: default(),
            rigid_body: RigidBody::Dynamic,
            locked_axes: LockedAxes::ROTATION_LOCKED,
            impulse: default(),
            velocity: default(),
            dominance: default(),
        }
    }
}

impl CharacterControllerBundle {
    pub(crate) fn capsule(height: f32, radius: f32) -> Self {
        Self {
            collider: Collider::capsule_y(height / 2., radius),
            ..default()
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
    /// Acceleration on the ground
    pub(crate) ground_acceleration: f32,
    /// Acceleration on the ground when[`Walking::sprinting`] is `true`
    pub(crate) sprinting_acceleration: f32,
    /// Acceleration in the air
    pub(crate) aerial_acceleration: f32,
    /// Acceleration in opposide direction of velocity when not explicitely walking, i.e. [`Walking::direction`] is [`Option::None`]
    pub(crate) braking_acceleration: f32,
    /// Speed at which we stop braking and just set the horizontal velocity to 0
    pub(crate) stopping_speed: f32,
    /// Direction in which we want to walk this tick. When not normalized, the acceleration will be scaled accordingly.
    pub(crate) direction: Option<Vec3>,
    /// Whether we are sprinting this tick
    pub(crate) sprinting: bool,
}

impl Walking {
    pub(crate) fn get_acceleration(&self, grounded: bool) -> Option<Vec3> {
        let acceleration = if grounded {
            if self.sprinting {
                self.sprinting_acceleration
            } else {
                self.ground_acceleration
            }
        } else {
            self.aerial_acceleration
        };
        self.direction.map(|dir| dir * acceleration)
    }
}

impl Default for Walking {
    fn default() -> Self {
        Self {
            ground_acceleration: 14.,
            sprinting_acceleration: 19.,
            aerial_acceleration: 9.,
            braking_acceleration: 5.,
            stopping_speed: 0.1,
            direction: None,
            sprinting: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Component, Reflect, Default, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub(crate) struct Grounded(pub(crate) bool);

#[derive(Debug, Clone, PartialEq, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub(crate) struct Jumping {
    /// Speed of the jump in m/s
    pub(crate) speed: f32,
    /// Was jump requested?
    pub(crate) requested: bool,
}

impl Default for Jumping {
    fn default() -> Self {
        Self {
            speed: 3.5,
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
