use crate::actions::Actions;
use crate::loading::MaterialAssets;
use crate::GameState;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use smooth_bevy_cameras::LookTransform;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Debug, Component, Default, Clone)]
pub struct CharacterVelocity(Vect);

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

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<MaterialAssets>,
) {
    let texture_size = 256.0;
    commands.spawn((
        RigidBody::KinematicVelocityBased,
        Collider::ball(texture_size / 2.),
        KinematicCharacterController {
            // Don’t allow climbing slopes larger than 45 degrees.
            max_slope_climb_angle: 45.0_f32.to_radians() as Real,
            // Automatically slide down on slopes smaller than 30 degrees.
            min_slope_slide_angle: 30.0_f32.to_radians() as Real,
            // The character offset is set to 0.4 multiplied by the collider’s height.
            offset: CharacterLength::Absolute(1.0),
            // Snap to the ground if the vertical distance to the ground is smaller than 2.0.
            snap_to_ground: Some(CharacterLength::Absolute(2.0)),
            ..default()
        },
        Player,
        Grounded::default(),
        CharacterVelocity::default(),
        Jump::default(),
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: texture_size / 2.,
                ..default()
            })),
            material: materials.dirt.clone(),
            ..default()
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
        let min_gravity = g * 1.;
        let gravity = (g * dt).max(max_gravity).min(min_gravity);
        velocity.0.y += gravity;
    }
}

fn handle_jump(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<(&Grounded, &mut CharacterVelocity, &mut Jump), With<Player>>,
) {
    let y_speed = 1_100.0;
    let dt = time.delta_seconds();
    let jump_requested = actions.jump;
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
    camera_query: Query<&mut LookTransform>,
) {
    let dt = time.delta_seconds();
    let speed = 450.0;

    let camera = match camera_query.iter().next() {
        Some(transform) => transform,
        None => return,
    };
    let forward = (camera.target - camera.eye)
        .xz()
        .try_normalize()
        .unwrap_or(Vec2::Y);
    let sideward = forward.perp();
    let y_action = actions.player_movement.map(|mov| mov.y).unwrap_or_default();
    let x_action = actions.player_movement.map(|mov| mov.x).unwrap_or_default();
    let forward_action = forward * y_action;
    let sideward_action = sideward * x_action;
    let movement = (forward_action + sideward_action) * speed * dt;

    for (mut velocity,) in &mut player_query {
        velocity.0.x += movement.x;
        velocity.0.z += movement.y;
    }
}

fn apply_velocity(
    mut player_query: Query<
        (
            &mut CharacterVelocity,
            &mut KinematicCharacterController,
            Option<&KinematicCharacterControllerOutput>,
        ),
        With<Player>,
    >,
) {
    for (mut velocity, mut controller, output) in &mut player_query {
        if let Some(output) = output {
            let epsilon = 0.0001;
            if output.effective_translation.x.abs() < epsilon && velocity.0.x.abs() > epsilon {
                if output.desired_translation.x < 0.0 {
                    velocity.0.x = velocity.0.x.max(0.0)
                } else if output.desired_translation.x > 0.0 {
                    velocity.0.x = velocity.0.x.min(0.0)
                }
            }
        }

        controller.translation = Some(velocity.0);
        velocity.0 = default();
    }
}
