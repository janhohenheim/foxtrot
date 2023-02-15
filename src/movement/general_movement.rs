use anyhow::{Context, Result};
use bevy::prelude::*;

use bevy_rapier3d::prelude::*;
mod components;
use crate::level_instantiation::spawning::AnimationEntityLink;
use crate::util::log_error::log_errors;
use crate::util::trait_extension::Vec3Ext;
use crate::GameState;
pub use components::{Velocity, *};

/// Handles movement of kinematic character controllers, i.e. entities with the [`KinematicCharacterBundle`]. A movement is done by applying forces to the objects.
/// The default forces on a character going right are:  
/// ```text
/// ┌──────────────────────────────┐
/// │            Gravity           │
/// │               ↓              │
/// │              ╔═╗             │
/// │   Walking ─► ║ ║ ◄─ Drag     │
/// │              ╚═╝             │  
/// │                              │
/// └──────────────────────────────┘
/// ```
/// All physics values are assumed to be in SI units, e.g. forces are measured in N and acceleration in m/s².
///
/// The [`Walking`] and [`Jumping`] components are user friendly ways of influencing the corresponding forces.
/// There is no explicit maximum speed since the [`Drag`] counteracts all other forces until reaching an equilibrium.
/// The [`Grounded`] component is used to determine whether the character is on the ground or not.
/// To influence movement, apply your force by adding it to the character's total [`Force`]. Common ways to do this are:
/// - A continuous force like walking: `force.0 += acceleration * mass.0`, with `force`: [`Force`], `mass`: [`Mass`], and a user-defined `acceleration`: [`f32`]
/// - An instantaneous force (i.e. an impulse) like jumping: `force.0 += velocity * mass.0 / time.delta_seconds()`, with `force`: [`Force`], `mass`: [`Mass`], `time`: [`Res<Time>`](Time) and a user-defined `velocity`: [`f32`]
///
/// Note: you might notice that the normal force is not included in the above diagram. This is because the underlying [`KinematicCharacterController`] takes care of the character not penetrating colliders, thus emulating this force.
pub struct GeneralMovementPlugin;

impl Plugin for GeneralMovementPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Model>()
            .register_type::<Grounded>()
            .register_type::<Jumping>()
            .register_type::<Velocity>()
            .register_type::<Drag>()
            .register_type::<Walking>()
            .register_type::<Force>()
            .register_type::<Mass>()
            .register_type::<Gravity>()
            .register_type::<CharacterAnimations>()
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(update_grounded)
                    .with_system(apply_gravity.after(update_grounded).before(apply_force))
                    .with_system(apply_walking.after(update_grounded).before(apply_force))
                    .with_system(apply_jumping.after(apply_gravity).before(apply_force))
                    .with_system(
                        apply_drag
                            .after(apply_walking)
                            .after(apply_jumping)
                            .before(apply_force),
                    )
                    .with_system(apply_force)
                    .with_system(reset_movement_components.after(apply_force))
                    .with_system(rotate_characters)
                    .with_system(play_animations.pipe(log_errors)),
            );
    }
}

fn update_grounded(
    mut query: Query<(
        &mut Grounded,
        &Velocity,
        &KinematicCharacterController,
        Option<&KinematicCharacterControllerOutput>,
    )>,
) {
    for (mut grounded, velocity, controller, output) in &mut query {
        let falling = velocity.0.dot(controller.up) < -1e-5;
        if !falling {
            grounded.force_set(false)
        } else if let Some(output) = output {
            grounded.try_set(output.grounded);
        }
    }
}

fn apply_gravity(
    mut character: Query<(&mut Force, &KinematicCharacterController, &Mass, &Gravity)>,
) {
    for (mut force, controller, mass, gravity) in &mut character {
        let gravitational_force = -controller.up * gravity.0 * mass.0;
        force.0 += gravitational_force;
    }
}

/// Treat `Force` as readonly after this system.
pub fn apply_force(
    time: Res<Time>,
    mut player_query: Query<(
        &Force,
        &mut Velocity,
        &mut KinematicCharacterController,
        &Mass,
    )>,
) {
    let dt = time.delta_seconds();
    for (force, mut velocity, mut controller, mass) in &mut player_query {
        let acceleration = force.0 / mass.0;
        let desired_translation =
            (velocity.0 * dt + 0.5 * acceleration * dt * dt).collapse_approx_zero();
        velocity.0 += acceleration * dt;
        controller.translation = Some(desired_translation);
    }
}

pub fn reset_movement_components(
    mut forces: Query<&mut Force>,
    mut walking: Query<&mut Walking>,
    mut jumpers: Query<&mut Jumping>,
    mut velocities: Query<&mut Velocity>,
) {
    for mut force in &mut forces {
        force.0 = Vec3::ZERO;
    }
    for mut walk in &mut walking {
        walk.direction = None;
    }
    for mut jumper in &mut jumpers {
        jumper.requested = false;
    }
    for mut velocity in &mut velocities {
        velocity.0 = velocity.0.collapse_approx_zero();
    }
}

pub fn apply_jumping(
    time: Res<Time>,
    mut character_query: Query<(
        &Grounded,
        &mut Force,
        &mut Velocity,
        &KinematicCharacterController,
        &Mass,
        &Jumping,
    )>,
) {
    let dt = time.delta_seconds();
    for (grounded, mut force, mut velocity, controller, mass, jump) in &mut character_query {
        if jump.requested && grounded.is_grounded() {
            force.0 += controller.up * mass.0 * jump.speed / dt;

            // Kill any downward velocity. This ensures that repeated jumps are always the same height.
            // Otherwise the falling velocity from the last tick would dampen the jump velocity.
            let velocity_components = velocity.0.split(controller.up);
            velocity.0 = velocity_components.horizontal;
        }
    }
}

fn rotate_characters(
    time: Res<Time>,
    mut player_query: Query<(
        &KinematicCharacterControllerOutput,
        &KinematicCharacterController,
        &mut Transform,
    )>,
) {
    let dt = time.delta_seconds();
    for (output, controller, mut transform) in player_query.iter_mut() {
        let horizontal_movement = output.effective_translation.split(controller.up).horizontal;
        if horizontal_movement.is_approx_zero() {
            continue;
        }
        let target_transform =
            transform.looking_at(transform.translation + horizontal_movement, controller.up);
        // Asymptotic averaging
        const SMOOTHNESS: f32 = 4.;
        let scale = (SMOOTHNESS * dt).min(1.);
        let rotation = transform.rotation.slerp(target_transform.rotation, scale);
        transform.rotation = rotation;
    }
}

fn play_animations(
    mut animation_player: Query<&mut AnimationPlayer>,
    characters: Query<(
        &KinematicCharacterControllerOutput,
        &KinematicCharacterController,
        &Grounded,
        &AnimationEntityLink,
        &CharacterAnimations,
    )>,
) -> Result<()> {
    for (output, controller, grounded, animation_entity_link, animations) in characters.iter() {
        let mut animation_player = animation_player
            .get_mut(animation_entity_link.0)
            .context("animation_entity_link held entity without animation player")?;

        let has_horizontal_movement = !output
            .effective_translation
            .split(controller.up)
            .horizontal
            .is_approx_zero();

        if !grounded.is_grounded() {
            animation_player
                .play(animations.aerial.clone_weak())
                .repeat();
        } else if has_horizontal_movement {
            animation_player.play(animations.walk.clone_weak()).repeat();
        } else {
            animation_player.play(animations.idle.clone_weak()).repeat();
        }
    }
    Ok(())
}

fn apply_drag(
    mut character_query: Query<(&mut Force, &Velocity, &KinematicCharacterController, &Drag)>,
) {
    for (mut force, velocity, controller, drag) in &mut character_query {
        let drag_force = drag.calculate_force(velocity.0, controller.up);
        force.0 += drag_force;
    }
}

pub fn apply_walking(
    mut character_query: Query<(
        &mut Force,
        &Walking,
        &mut Velocity,
        &KinematicCharacterController,
        &Grounded,
        &Mass,
    )>,
) {
    for (mut force, walking, mut velocity, controller, grounded, mass) in &mut character_query {
        if let Some(acceleration) = walking.get_acceleration(grounded.is_grounded()) {
            let walking_force = acceleration * mass.0;
            force.0 += walking_force;
        } else if grounded.is_grounded() {
            let velocity_components = velocity.0.split(controller.up);
            if velocity_components.horizontal.length_squared()
                < walking.stopping_speed * walking.stopping_speed
            {
                velocity.0 = velocity_components.vertical;
            } else if let Some(braking_direction) =
                velocity_components.horizontal.try_normalize().map(|v| -v)
            {
                let braking_force = walking.braking_acceleration * braking_direction * mass.0;
                force.0 += braking_force;
            }
        }
    }
}
