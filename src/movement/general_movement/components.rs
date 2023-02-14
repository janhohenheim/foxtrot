use crate::util::trait_extension::Vec3Ext;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use serde::{Deserialize, Serialize};
use std::f32::consts::TAU;

#[derive(Debug, Clone, Bundle)]
pub struct KinematicCharacterBundle {
    pub velocity: Velocity,
    pub force: Force,
    pub mass: Mass,
    pub walking: Walking,
    pub jumping: Jumping,
    pub grounded: Grounded,
    pub drag: Drag,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub character_controller: KinematicCharacterController,
    pub gravity: Gravity,
}

impl Default for KinematicCharacterBundle {
    fn default() -> Self {
        Self {
            velocity: default(),
            force: default(),
            mass: default(),
            walking: default(),
            jumping: default(),
            grounded: default(),
            drag: default(),
            collider: default(),
            gravity: default(),
            rigid_body: RigidBody::KinematicVelocityBased,
            character_controller: KinematicCharacterController {
                offset: CharacterLength::Relative(0.05),
                ..default()
            },
        }
    }
}

impl KinematicCharacterBundle {
    pub fn capsule(height: f32, radius: f32) -> Self {
        Self {
            collider: Collider::capsule_y(height / 2., radius),
            drag: Drag::for_capsule(height, radius),
            ..default()
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Model;

#[derive(Debug, Clone, Copy, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Velocity(pub Vect);

#[derive(Debug, Clone, Copy, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Force(pub Vect);

#[derive(Debug, Clone, Copy, PartialEq, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Mass(pub f32);

impl Default for Mass {
    fn default() -> Self {
        Self(3.0)
    }
}

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

#[derive(Debug, Clone, PartialEq, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Drag {
    pub fluid_density: f32,
    pub area: f32,
    pub drag_coefficient: f32,
}

impl Drag {
    pub fn for_capsule(height: f32, radius: f32) -> Self {
        let cross_sectional_area = (height - radius) * height + TAU * radius * radius;
        Self {
            area: cross_sectional_area,
            ..default()
        }
    }

    pub fn calculate_force(&self, velocity: Vec3, up: Vec3) -> Vec3 {
        velocity
            .split(up)
            .as_array()
            .iter()
            .map(|&v| self.calculate_force_for_component(v))
            .sum()
    }

    fn calculate_force_for_component(&self, velocity: Vec3) -> Vec3 {
        let speed_squared = velocity.length_squared();
        if speed_squared < 1e-5 {
            return Vec3::ZERO;
        }
        0.5 * self.fluid_density
            * self.area
            * self.drag_coefficient
            * speed_squared
            * -velocity.normalize()
    }
}

impl Default for Drag {
    fn default() -> Self {
        Self {
            // dry air at 20°C, see <https://en.wikipedia.org/wiki/Density_of_air#Dry_air>
            fluid_density: 1.2041,
            // Arbitrary
            area: 1.0,
            // Person, see <https://www.engineeringtoolbox.com/drag-coefficient-d_627.html>
            drag_coefficient: 1.2,
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
/// Gravity constant in m/s²
pub struct Gravity(pub f32);

impl Default for Gravity {
    fn default() -> Self {
        Self(9.81)
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
