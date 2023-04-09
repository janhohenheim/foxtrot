use anyhow::{Context, Result};
use bevy::prelude::*;
use std::time::Duration;

use bevy_rapier3d::prelude::*;
mod components;
use crate::file_system_interaction::config::GameConfig;
use crate::level_instantiation::spawning::AnimationEntityLink;
use crate::util::smoothness_to_lerp_factor;
use crate::util::trait_extension::{TransformExt, Vec3Ext};
use crate::GameState;
use bevy_mod_sysfail::macros::*;
pub(crate) use components::*;

/// Handles movement of character controllers, i.e. entities with the [`CharacterControllerBundle`].
/// The default forces on a character going right are:  
/// ```text
/// ┌──────────────────────────────┐
/// │            Gravity           │
/// │               ↓              │
/// │              ╔═╗             │
/// │   Walking ─► ║ ║ ◄─ Damping  │
/// │              ╚═╝             │  
/// │                              │
/// └──────────────────────────────┘
/// ```
/// All physics values are assumed to be in SI units, e.g. forces are measured in N and acceleration in m/s².
///
/// The [`Walking`] and [`Jumping`] components are user friendly ways of influencing the corresponding forces.
/// There is no explicit maximum speed since the damping counteracts all other forces until reaching an equilibrium.
/// The [`Grounded`] component is used to determine whether the character is on the ground or not.
/// To influence movement, apply your force by adding it to the character's total [`ExternalForce`] or [`ExternalImpulse`]. This is usually done like this:
/// - A continuous force like walking: `external_force.force += acceleration * read_mass_properties.0.mass`, with `external_force`: [`ExternalForce`], `read_mass_properties`: [`ReadMassProperties`], and a user-defined `acceleration`: [`Vec3`]
/// - An instantaneous force (i.e. an impulse) like jumping: `external_impulse.impulse += velocity * read_mass_properties.0.mass`, with `external_impulse`: [`ExternalImpulse`], `read_mass_properties`: [`ReadMassProperties`], and a user-defined `velocity`: [`Vec3`]
///
/// Note: you might notice that the normal force is not included in the above diagram. This is because rapier emulates it by moving penetrating colliders out of each other.
pub(crate) fn general_movement_plugin(app: &mut App) {
    app.register_type::<Grounded>()
        .register_type::<Jumping>()
        .register_type::<Velocity>()
        .register_type::<Walking>()
        .register_type::<CharacterAnimations>()
        .add_systems(
            (
                reset_forces_and_impulses,
                update_grounded,
                apply_jumping,
                apply_walking,
                rotate_characters,
                play_animations,
                sync_models,
                reset_movement_components,
            )
                .chain()
                .in_set(GeneralMovementSystemSet)
                .in_set(OnUpdate(GameState::Playing)),
        );
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub(crate) struct GeneralMovementSystemSet;

fn update_grounded(
    mut query: Query<(Entity, &Transform, &Collider, &mut Grounded)>,
    rapier_context: Res<RapierContext>,
) {
    #[cfg(feature = "tracing")]
    let _span = info_span!("update_grounded").entered();
    for (entity, transform, collider, mut grounded) in &mut query {
        let height = collider.raw.compute_local_aabb().maxs.y;
        grounded.0 = rapier_context
            .cast_ray(
                transform.translation,
                transform.down(),
                height + 0.1,
                true,
                QueryFilter::new()
                    .exclude_collider(entity)
                    .exclude_sensors(),
            )
            .is_some();
    }
}

pub(crate) fn reset_forces_and_impulses(
    mut forces: Query<&mut ExternalForce>,
    mut impulses: Query<&mut ExternalImpulse>,
) {
    #[cfg(feature = "tracing")]
    let _span = info_span!("reset_forces_and_impulses").entered();
    for mut force in &mut forces {
        *force = default();
    }
    for mut impulse in &mut impulses {
        *impulse = default();
    }
}

pub(crate) fn reset_movement_components(
    mut walking: Query<&mut Walking>,
    mut jumpers: Query<&mut Jumping>,
) {
    #[cfg(feature = "tracing")]
    let _span = info_span!("reset_movement_components").entered();
    for mut walk in &mut walking {
        walk.direction = None;
    }
    for mut jumper in &mut jumpers {
        jumper.requested = false;
    }
}

pub(crate) fn apply_jumping(
    mut character_query: Query<(
        &Grounded,
        &mut ExternalImpulse,
        &mut Velocity,
        &ReadMassProperties,
        &Jumping,
        &Transform,
    )>,
) {
    #[cfg(feature = "tracing")]
    let _span = info_span!("apply_jumping").entered();
    for (grounded, mut impulse, mut velocity, mass, jump, transform) in &mut character_query {
        if jump.requested && grounded.0 {
            let up = transform.up();
            impulse.impulse += up * mass.0.mass * jump.speed;

            // Kill any downward velocity. This ensures that repeated jumps are always the same height.
            // Otherwise the falling velocity from the last tick would dampen the jump velocity.
            let velocity_components = velocity.linvel.split(up);
            velocity.linvel = velocity_components.horizontal;
        }
    }
}

fn rotate_characters(
    time: Res<Time>,
    mut player_query: Query<(&Velocity, &mut Transform)>,
    config: Res<GameConfig>,
) {
    #[cfg(feature = "tracing")]
    let _span = info_span!("rotate_characters").entered();
    let dt = time.delta_seconds();
    for (velocity, mut transform) in player_query.iter_mut() {
        let up = transform.up();
        let horizontal_movement = velocity.linvel.split(up).horizontal;
        if horizontal_movement.is_approx_zero() {
            continue;
        }
        let target_transform =
            transform.looking_at(transform.translation + horizontal_movement, up);
        // Asymptotic averaging
        let smoothness = config.characters.rotation_smoothing;
        let factor = smoothness_to_lerp_factor(smoothness, dt);
        let rotation = transform.rotation.slerp(target_transform.rotation, factor);
        transform.rotation = rotation;
    }
}

#[sysfail(log(level = "error"))]
fn play_animations(
    mut animation_player: Query<&mut AnimationPlayer>,
    characters: Query<(
        &Velocity,
        &Transform,
        &Grounded,
        &AnimationEntityLink,
        &CharacterAnimations,
    )>,
) -> Result<()> {
    #[cfg(feature = "tracing")]
    let _span = info_span!("play_animations").entered();
    for (velocity, transform, grounded, animation_entity_link, animations) in characters.iter() {
        let mut animation_player = animation_player
            .get_mut(animation_entity_link.0)
            .context("animation_entity_link held entity without animation player")?;

        let has_horizontal_movement = !velocity
            .linvel
            .split(transform.up())
            .horizontal
            .is_approx_zero();

        if !grounded.0 {
            animation_player
                .play_with_transition(animations.aerial.clone_weak(), Duration::from_secs_f32(0.2))
                .repeat();
        } else if has_horizontal_movement {
            animation_player
                .play_with_transition(animations.walk.clone_weak(), Duration::from_secs_f32(0.2))
                .repeat();
        } else {
            animation_player
                .play_with_transition(animations.idle.clone_weak(), Duration::from_secs_f32(0.2))
                .repeat();
        }
    }
    Ok(())
}

pub(crate) fn apply_walking(
    mut character_query: Query<(
        &mut ExternalForce,
        &Walking,
        &mut Velocity,
        &Grounded,
        &ReadMassProperties,
        &Transform,
    )>,
) {
    #[cfg(feature = "tracing")]
    let _span = info_span!("apply_walking").entered();
    for (mut force, walking, mut velocity, grounded, mass, transform) in &mut character_query {
        let mass = mass.0.mass;
        if let Some(acceleration) = walking.get_acceleration(grounded.0) {
            let walking_force = acceleration * mass;
            force.force += walking_force;
        } else if grounded.0 {
            let velocity_components = velocity.linvel.split(transform.up());
            if velocity_components.horizontal.length_squared()
                < walking.stopping_speed * walking.stopping_speed
            {
                velocity.linvel = velocity_components.vertical;
            } else if let Some(braking_direction) =
                velocity_components.horizontal.try_normalize().map(|v| -v)
            {
                let braking_force = walking.braking_acceleration * braking_direction * mass;
                force.force += braking_force;
            }
        }
    }
}

#[sysfail(log(level = "error"))]
fn sync_models(
    time: Res<Time>,
    mut commands: Commands,
    without_model: Query<(&Transform, &Visibility), Without<Model>>,
    mut with_model: Query<(Entity, &mut Transform, &mut Visibility, &Model)>,
    game_config: Res<GameConfig>,
) -> Result<()> {
    let dt = time.delta_seconds();
    for (model_entity, mut model_transform, mut visibility, model) in with_model.iter_mut() {
        if let Ok((target_transform, target_visibility)) = without_model.get(model.target) {
            let smoothness = game_config.characters.model_sync_smoothing;
            let factor = smoothness_to_lerp_factor(smoothness, dt);
            *model_transform = model_transform.lerp(*target_transform, factor);
            *visibility = *target_visibility;
        } else {
            commands.entity(model_entity).despawn_recursive();
        }
    }
    Ok(())
}
