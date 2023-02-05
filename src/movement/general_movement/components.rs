use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use serde::{Deserialize, Serialize};
use std::f32::consts::TAU;

#[derive(Debug, Clone, Bundle)]
pub struct KinematicCharacterBundle {
    pub velocity: Velocity,
    pub force: Force,
    pub mass: Mass,
    pub walker: Walker,
    pub jump: Jump,
    pub grounded: Grounded,
    pub drag: Drag,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub character_controller: KinematicCharacterController,
}

impl Default for KinematicCharacterBundle {
    fn default() -> Self {
        Self {
            velocity: default(),
            force: default(),
            mass: default(),
            walker: default(),
            jump: default(),
            grounded: default(),
            drag: default(),
            collider: default(),
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
        Self(1.0)
    }
}

#[derive(Debug, Clone, PartialEq, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Walker {
    pub ground_acceleration: f32,
    pub aerial_acceleration: f32,
    pub direction: Option<Vec3>,
}

impl Walker {
    pub fn calculate_acceleration(&self, grounded: bool) -> Option<Vec3> {
        let acceleration = if grounded {
            self.ground_acceleration
        } else {
            self.aerial_acceleration
        };
        self.direction.map(|dir| dir * acceleration)
    }
}

impl Default for Walker {
    fn default() -> Self {
        Self {
            ground_acceleration: 0.1,
            aerial_acceleration: 0.01,
            direction: None,
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
    pub fn calculate_force(&self, velocity: Vec3) -> Vec3 {
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
            // Empty
            area: 0.0,
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
}

#[derive(Debug, Clone, PartialEq, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Jump {
    pub time_since_start: Timer,
    pub state: JumpState,
    pub g: f32,
    pub duration: f32,
    pub speed: f32,
}

impl Default for Jump {
    fn default() -> Self {
        Self {
            time_since_start: Timer::with_max_time(),
            state: default(),
            g: 0.3,
            duration: 0.23,
            speed: 0.95,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub enum JumpState {
    InProgress,
    Done,
}
impl Default for JumpState {
    fn default() -> Self {
        Self::Done
    }
}
impl Jump {
    pub fn speed_fraction(&self) -> f32 {
        let t: f32 = self.time_since_start.into();
        // shifted and scaled sigmoid
        let suggestion = 1. / (1. + (40. * (t - 0.1)).exp());
        if suggestion > 0.001 {
            suggestion
        } else {
            0.0
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

#[derive(Debug, Clone, Copy, PartialEq, Component, Reflect, Default, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Timer {
    elapsed_time: f32,
}

impl From<Timer> for f32 {
    fn from(timer: Timer) -> Self {
        timer.elapsed_time
    }
}

impl Timer {
    pub fn with_max_time() -> Self {
        Self {
            elapsed_time: f32::MAX,
        }
    }
    pub fn start(&mut self) {
        self.elapsed_time = 0.0
    }
    pub fn update(&mut self, dt: f32) {
        self.elapsed_time = if self.elapsed_time < f32::MAX - dt - 0.1 {
            self.elapsed_time + dt
        } else {
            f32::MAX
        }
    }
    pub fn is_active(&self) -> bool {
        self.elapsed_time > 1e-5
    }
}
