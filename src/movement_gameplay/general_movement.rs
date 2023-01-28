use bevy::prelude::*;

use bevy_rapier3d::prelude::*;
mod components;
use crate::level_design::spawning::AnimationEntityLink;
use crate::util::trait_extension::Vec3Ext;
use crate::GameState;
pub use components::*;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Model>()
            .register_type::<JumpState>()
            .register_type::<Grounded>()
            .register_type::<Jump>()
            .register_type::<CharacterVelocity>()
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(update_grounded.label("update_grounded"))
                    .with_system(
                        apply_gravity
                            .label("apply_gravity")
                            .after("update_grounded")
                            .before("apply_velocity"),
                    )
                    .with_system(apply_velocity.label("apply_velocity"))
                    .with_system(
                        reset_velocity
                            .label("reset_velocity")
                            .after("apply_velocity"),
                    )
                    .with_system(rotate_model.label("rotate_model")),
            );
    }
}

fn update_grounded(
    time: Res<Time>,
    mut query: Query<(&mut Grounded, &KinematicCharacterControllerOutput)>,
) {
    let dt = time.delta_seconds();
    for (mut grounded, output) in &mut query {
        if output.grounded {
            grounded.time_since_last_grounded.start()
        } else {
            grounded.time_since_last_grounded.update(dt)
        }
    }
}

fn apply_gravity(mut player_query: Query<(&mut CharacterVelocity, &Grounded, &Jump)>) {
    for (mut velocity, grounded, jump) in &mut player_query {
        if matches!(jump.state, JumpState::InProgress) {
            continue;
        }
        let dt = f32::from(grounded.time_since_last_grounded)
            - f32::from(jump.time_since_start).min(jump.duration);
        let max_gravity = jump.g * 5.;
        let min_gravity = jump.g * 0.1;
        // min and max look swapped because gravity is negative
        let gravity = (jump.g * dt).clamp(max_gravity, min_gravity);
        velocity.0.y += gravity;
    }
}

/// Treat `CharacterVelocity` as readonly after this system.
fn apply_velocity(
    mut player_query: Query<(&CharacterVelocity, &mut KinematicCharacterController)>,
) {
    for (velocity, mut controller) in &mut player_query {
        controller.translation = Some(velocity.0);
    }
}

fn reset_velocity(mut player_query: Query<&mut CharacterVelocity>) {
    for mut velocity in &mut player_query {
        velocity.0 = default();
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
        transforms
            .get_mut(link.0)
            .unwrap()
            .look_at(horizontal_movement, Vec3::Y);
    }
}
