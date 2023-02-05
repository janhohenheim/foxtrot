use bevy::prelude::*;

use bevy_rapier3d::prelude::*;
mod components;
use crate::level_instanciation::spawning::AnimationEntityLink;
use crate::util::trait_extension::Vec3Ext;
use crate::GameState;
pub use components::{Velocity, *};

pub struct GeneralMovementPlugin;

impl Plugin for GeneralMovementPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Model>()
            .register_type::<JumpState>()
            .register_type::<Grounded>()
            .register_type::<Jump>()
            .register_type::<Velocity>()
            .register_type::<Drag>()
            .register_type::<Walker>()
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(update_grounded.label("update_grounded"))
                    .with_system(
                        apply_gravity
                            .label("apply_gravity")
                            .after("update_grounded")
                            .before("apply_force"),
                    )
                    .with_system(
                        apply_walking
                            .label("apply_walking")
                            .after("update_grounded")
                            .before("apply_force"),
                    )
                    .with_system(
                        apply_drag
                            .label("apply_drag")
                            .after("apply_walking")
                            .after("apply_gravity")
                            .before("apply_force"),
                    )
                    .with_system(apply_force.label("apply_force"))
                    .with_system(reset_force.label("reset_force").after("apply_force"))
                    .with_system(
                        reset_walking_direction
                            .label("reset_walking_direction")
                            .after("apply_force"),
                    )
                    .with_system(rotate_model)
                    .with_system(play_animations),
            );
    }
}

fn update_grounded(mut query: Query<(&mut Grounded, &KinematicCharacterControllerOutput)>) {
    for (mut grounded, output) in &mut query {
        grounded.try_set(output.grounded);
    }
}

fn apply_gravity(mut character: Query<(&mut Force, &KinematicCharacterController, &Mass, &Jump)>) {
    for (mut force, controller, mass, jump) in &mut character {
        let gravitational_force = -controller.up * jump.g * mass.0;
        force.0 += gravitational_force;
    }
}

/// Treat `Force` as readonly after this system.
fn apply_force(
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
        let desired_translation = velocity.0 * dt + 0.5 * acceleration * dt * dt;
        velocity.0 += acceleration * dt;
        controller.translation = Some(desired_translation);
    }
}

fn reset_force(mut player_query: Query<&mut Force>) {
    for mut force in &mut player_query {
        force.0 = Vec3::ZERO;
    }
}

fn reset_walking_direction(mut character_query: Query<&mut Walker>) {
    for mut walker in &mut character_query {
        walker.direction = None;
    }
}

fn rotate_model(
    player_query: Query<(&KinematicCharacterControllerOutput, &AnimationEntityLink)>,
    mut transforms: Query<&mut Transform>,
) {
    for (output, link) in player_query.iter() {
        let horizontal_movement = output.effective_translation.x0z();
        if horizontal_movement.is_approx_zero() {
            continue;
        }
        let mut transform = transforms.get_mut(link.0).unwrap();
        *transform = transform.looking_at(transform.translation + horizontal_movement, Vec3::Y);
    }
}

fn play_animations(
    mut animation_player: Query<&mut AnimationPlayer>,
    characters: Query<(
        &KinematicCharacterControllerOutput,
        &Grounded,
        &AnimationEntityLink,
        &CharacterAnimations,
    )>,
) {
    for (output, grounded, animation_entity_link, animations) in characters.iter() {
        let mut animation_player = animation_player
            .get_mut(animation_entity_link.0)
            .expect("animation_entity_link held entity without animation player");

        let has_horizontal_movement = !output.effective_translation.x0z().is_approx_zero();

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
}

fn apply_drag(mut character_query: Query<(&mut Force, &Velocity, &Drag)>) {
    for (mut force, velocity, drag) in &mut character_query {
        let drag_force = drag.calculate_force(velocity.0);
        force.0 += drag_force;
    }
}

fn apply_walking(mut character_query: Query<(&mut Force, &Walker, &Grounded, &Mass)>) {
    for (mut force, walker, grounded, mass) in &mut character_query {
        if let Some(acceleration) = walker.calculate_acceleration(grounded.is_grounded()) {
            let walking_force = acceleration * mass.0;
            force.0 += walking_force;
        }
    }
}
