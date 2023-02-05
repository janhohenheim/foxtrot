use bevy::prelude::*;

use bevy_rapier3d::prelude::*;
mod components;
use crate::level_instanciation::spawning::AnimationEntityLink;
use crate::player_control::player_embodiment::Player;
use crate::util::trait_extension::Vec3Ext;
use crate::GameState;
pub use components::*;

pub struct GeneralMovementPlugin;

impl Plugin for GeneralMovementPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Model>()
            .register_type::<JumpState>()
            .register_type::<Grounded>()
            .register_type::<Jump>()
            .register_type::<CharacterVelocity>()
            .register_type::<Drag>()
            .register_type::<Walker>()
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(update_grounded.label("update_grounded"))
                    .with_system(
                        apply_gravity
                            .label("apply_gravity")
                            .after("update_grounded")
                            .before("apply_velocity"),
                    )
                    .with_system(
                        apply_walking
                            .label("apply_walking")
                            .after("update_grounded")
                            .before("apply_velocity"),
                    )
                    .with_system(
                        apply_drag
                            .label("apply_drag")
                            .after("apply_walking")
                            .after("apply_gravity")
                            .before("apply_velocity"),
                    )
                    .with_system(apply_velocity.label("apply_velocity"))
                    .with_system(
                        reset_velocity
                            .label("reset_velocity")
                            .after("apply_velocity"),
                    )
                    .with_system(
                        reset_acceleration
                            .label("reset_acceleration")
                            .after("apply_velocity"),
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

fn apply_gravity(time: Res<Time>, mut character: Query<(&mut CharacterVelocity, &Jump)>) {
    let dt = time.delta_seconds();
    for (mut velocity, jump) in &mut character {
        let gravity = jump.g * dt;
        velocity.0.y += gravity;
    }
}

/// Treat `CharacterVelocity` as readonly after this system.
fn apply_velocity(
    mut player_query: Query<(
        &CharacterVelocity,
        &mut KinematicCharacterController,
        Option<&Player>,
    )>,
) {
    for (velocity, mut controller, player) in &mut player_query {
        let velocity = velocity.0;
        if player.is_some() {
            info!("velocity: {}", velocity);
        }
        controller.translation = Some(velocity);
    }
}

fn reset_velocity(mut player_query: Query<(&mut CharacterVelocity, &Grounded)>) {
    for (mut velocity, grounded) in &mut player_query {
        if grounded.is_grounded() {
            velocity.0.y = velocity.0.y.max(-0.1);
        }
        velocity.0 = velocity.0.collapse_approx_zero();
    }
}

fn reset_acceleration(mut character_query: Query<&mut Walker>) {
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

fn apply_drag(
    time: Res<Time>,
    mut character_query: Query<(&mut CharacterVelocity, &Drag, Option<&Player>)>,
) {
    let dt = time.delta_seconds();
    for (mut velocity, drag, player) in &mut character_query {
        let force = drag.calculate_force(velocity.0);
        if player.is_some() {
            info!("drag force: {}", force);
        }
        velocity.0 -= force * dt;
    }
}

fn apply_walking(
    time: Res<Time>,
    mut character_query: Query<(&mut CharacterVelocity, &Walker, &Grounded, Option<&Player>)>,
) {
    let dt = time.delta_seconds();
    for (mut velocity, walker, grounded, player) in &mut character_query {
        if let Some(acceleration) = walker.calculate_acceleration(grounded.is_grounded()) {
            if player.is_some() {
                info!("acceleration: {}", acceleration);
            }
            velocity.0 += acceleration * dt;
        }
    }
}
