use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Debug, Component, Default, Clone)]
pub struct CharacterVelocity(Vec2);

#[derive(Component, Default)]
pub struct Grounded {
    time_since_last_grounded: Timer,
}

#[derive(Component, Default)]
pub struct Jump {
    time_since_start: Timer,
}
impl Jump {
    pub fn speed_fraction(&self) -> f32 {
        let t: f32 = self.time_since_start.into();
        // shifted and scaled sigmoid
        let suggestion = 1. / (1. + (6. * (t - 1. / 2.)).exp());
        if suggestion > 0.001 {
            suggestion
        } else {
            0.0
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Timer {
    elapsed_time: f32,
}
impl Default for Timer {
    fn default() -> Self {
        Self {
            elapsed_time: f32::MAX,
        }
    }
}

impl From<Timer> for f32 {
    fn from(timer: Timer) -> Self {
        timer.elapsed_time
    }
}

impl Timer {
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
}
/// This plugin handles player related stuff like movement
/// Player logic is only active during the State `GameState::Playing`
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_player))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(update_grounded.label("update_grounded"))
                    .with_system(
                        handle_jump
                            .after("update_grounded")
                            .before("apply_velocity"),
                    )
                    .with_system(
                        apply_gravity
                            .after("update_grounded")
                            .before("apply_velocity"),
                    )
                    .with_system(
                        handle_horizontal_movement
                            .after("update_grounded")
                            .before("apply_velocity"),
                    )
                    .with_system(apply_velocity.label("apply_velocity")),
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
        CharacterVelocity::default(),
        Jump::default(),
        SpriteBundle {
            texture: textures.bevy.clone(),
            transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
            ..Default::default()
        },
    ));
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

fn apply_gravity(mut player_query: Query<(&mut CharacterVelocity, &Grounded)>) {
    for (mut velocity, grounded) in &mut player_query {
        let dt = <Timer as Into<f32>>::into(grounded.time_since_last_grounded);
        let g = -9.81;
        let max_gravity = g * 5.;
        let gravity = (g * dt).max(max_gravity);
        velocity.0.y += gravity;
    }
}

fn handle_jump(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<(&Grounded, &mut CharacterVelocity, &mut Jump), With<Player>>,
) {
    let y_speed = 600.0;
    let dt = time.delta_seconds();
    let jump_requested = actions
        .player_movement
        .map(|movement| movement.y > 0.1)
        .unwrap_or_default();
    for (grounded, mut velocity, mut jump) in &mut player_query {
        if jump_requested && <Timer as Into<f32>>::into(grounded.time_since_last_grounded) < 0.00001
        {
            jump.time_since_start.start();
        } else {
            jump.time_since_start.update(dt);
        }
        velocity.0.y += jump.speed_fraction() * y_speed * dt
    }
}

fn handle_horizontal_movement(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<(&mut CharacterVelocity,), With<Player>>,
) {
    let dt = time.delta_seconds();
    let x_speed = 450.0;
    for (mut velocity,) in &mut player_query {
        velocity.0.x += actions.player_movement.map(|mov| mov.x).unwrap_or_default() * x_speed * dt
    }
}

fn apply_velocity(
    mut player_query: Query<
        (&mut CharacterVelocity, &mut KinematicCharacterController),
        With<Player>,
    >,
) {
    for (mut velocity, mut controller) in &mut player_query {
        controller.translation = Some(velocity.0);
        velocity.0 = default();
    }
}
