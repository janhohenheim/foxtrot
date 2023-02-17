use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Bundle)]
pub struct CharacterControllerBundle {
    pub gravity_scale: GravityScale,
    pub mass: ColliderMassProperties,
    pub read_mass: ReadMassProperties,
    pub walking: Walking,
    pub jumping: Jumping,
    pub grounded: Grounded,
    pub damping: Damping,
    pub rigid_body: RigidBody,
    pub locked_axes: LockedAxes,
    pub collider: Collider,
    pub force: ExternalForce,
    pub impulse: ExternalImpulse,
    pub velocity: Velocity,
    pub dominance: Dominance,
    pub up: Up,
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
            damping: default(),
            collider: default(),
            rigid_body: RigidBody::Dynamic,
            locked_axes: LockedAxes::ROTATION_LOCKED,
            impulse: default(),
            velocity: default(),
            dominance: default(),
            up: Up(Vec3::Y),
        }
    }
}

impl CharacterControllerBundle {
    pub fn capsule(height: f32, radius: f32) -> Self {
        Self {
            collider: Collider::capsule_y(height / 2., radius),
            ..default()
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Model;

#[derive(Debug, Clone, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Up(pub Vec3);

#[derive(Debug, Clone, PartialEq, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Walking {
    /// Acceleration on the ground
    pub ground_acceleration: f32,
    /// Acceleration on the ground when[`Walking::sprinting`] is `true`
    pub sprinting_acceleration: f32,
    /// Acceleration in the air
    pub aerial_acceleration: f32,
    /// Acceleration in opposide direction of velocity when not explicitely walking, i.e. [`Walking::direction`] is [`Option::None`]
    pub braking_acceleration: f32,
    /// Speed at which we stop braking and just set the horizontal velocity to 0
    pub stopping_speed: f32,
    /// Direction in which we want to walk this tick. When not normalized, the acceleration will be scaled accordingly.
    pub direction: Option<Vec3>,
    /// Whether we are sprinting this tick
    pub sprinting: bool,
}

impl Walking {
    pub fn get_acceleration(&self, grounded: bool) -> Option<Vec3> {
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
            ground_acceleration: 10.,
            sprinting_acceleration: 20.,
            aerial_acceleration: 7.,
            braking_acceleration: 5.,
            stopping_speed: 0.1,
            direction: None,
            sprinting: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Component, Reflect, Default, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Grounded {
    state: bool,
    wants_change: bool,
}

impl Grounded {
    pub fn is_grounded(&self) -> bool {
        self.state
    }

    /// Sets the grounded state to the given value after being requested to do so twice.
    /// This is to combat both false negatives when walking on the ground and false positives when jumping up a wall.
    pub fn try_set(&mut self, new_state: bool) {
        if self.state == new_state {
            self.wants_change = false
        } else if self.wants_change {
            self.state = new_state;
            self.wants_change = false;
        } else {
            self.wants_change = true;
        }
    }

    pub fn force_set(&mut self, new_state: bool) {
        self.state = new_state;
        self.wants_change = false;
    }
}

#[derive(Debug, Clone, PartialEq, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Jumping {
    /// Speed of the jump in m/s
    pub speed: f32,
    /// Was jump requested?
    pub requested: bool,
}

impl Default for Jumping {
    fn default() -> Self {
        Self {
            speed: 4.,
            requested: false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Component, Reflect, Default)]
#[reflect(Component)]
pub struct CharacterAnimations {
    pub idle: Handle<AnimationClip>,
    pub walk: Handle<AnimationClip>,
    pub aerial: Handle<AnimationClip>,
}
