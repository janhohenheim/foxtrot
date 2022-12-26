use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub struct Grounded {
    grounded: bool,
}

#[derive(Component, Default)]
pub struct Jump {
    time_since_start: f32,
}

/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_player))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(move_player.label("move_player"))
                    .with_system(add_gravity.label("add_gravity").after("move_player"))
                    .with_system(check_ground.after("add_gravity"))
                    .with_system(apply_jump.after("add_gravity")),
            );
    }
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn((
        RigidBody::KinematicVelocityBased,
        Collider::ball(0.5),
        KinematicCharacterController::default(),
        Player,
        Grounded::default(),
        Jump::default(),
        SpriteBundle {
            texture: textures.bevy.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..Default::default()
        },
    ));
}

fn check_ground(mut query: Query<(&mut Grounded, &KinematicCharacterControllerOutput)>) {
    for (mut grounded, output) in &mut query {
        grounded.grounded = output.grounded
    }
}

fn add_gravity(
    time: Res<Time>,
    mut player_query: Query<(&mut KinematicCharacterController, &Grounded)>,
) {
    for (mut controller, grounded) in &mut player_query {
        if grounded.grounded {
            continue;
        }
        let gravity = Vec2::new(0.0, -9.81 * 20.0 * time.delta_seconds());
        controller.translation = Some(
            controller
                .translation
                .map(|translation| translation + gravity)
                .unwrap_or(gravity),
        );
    }
}

fn apply_jump(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<(&Grounded, &mut Jump), With<Player>>,
) {
    let dt = time.delta_seconds();
    let jump_requested = actions
        .player_movement
        .map(|movement| movement.y > 0.001)
        .unwrap_or_default();
    for (grounded, mut jump) in &mut player_query {
        if jump_requested && grounded.grounded {
            jump.time_since_start = 0.0
        } else {
            jump.time_since_start += dt
        }
    }
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<(&Jump, &mut KinematicCharacterController), With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }
    let dt = time.delta_seconds();
    let x_speed = 300.0;
    let y_speed = 600.0;
    for (jump, mut controller) in &mut player_query {
        let movement = Vec2::new(
            actions.player_movement.unwrap().x * x_speed * dt,
            calculate_jump_speed(jump.time_since_start) * y_speed * dt,
        );
        controller.translation = Some(movement);
    }
}

fn calculate_jump_speed(t: f32) -> f32 {
    // shifted and scaled sigmoid
    let suggestion = 1. / (1. + (6. * (t - 1. / 2.)).exp());
    if suggestion > 0.001 {
        suggestion
    } else {
        0.0
    }
}
