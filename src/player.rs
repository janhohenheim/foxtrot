use crate::actions::Actions;
use crate::camera::PlayerCamera;
use crate::loading::{AnimationAssets, SceneAssets};
use crate::math::look_at;
use crate::GameState;
use bevy::gltf::Gltf;
use bevy::math::Vec3Swizzles;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

const G: f32 = -0.5;
const JUMP_DURATION: f32 = 0.23;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerSensor;

#[derive(Component)]
pub struct PlayerModel;

#[derive(Debug, Component, Default, Clone)]
pub struct CharacterVelocity(Vect);

#[derive(Component, Default)]
pub struct Grounded {
    time_since_last_grounded: Timer,
}

#[derive(Component, Default)]
pub struct Jump {
    time_since_start: Timer,
    state: JumpState,
}

#[derive(Debug)]
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
                    .with_system(link_animations)
                    .with_system(update_grounded.label("update_grounded"))
                    .with_system(
                        apply_gravity
                            .label("apply_gravity")
                            .after("update_grounded")
                            .before("apply_velocity"),
                    )
                    .with_system(handle_jump.after("apply_gravity").before("apply_velocity"))
                    .with_system(
                        handle_horizontal_movement
                            .after("update_grounded")
                            .before("apply_velocity"),
                    )
                    .with_system(apply_velocity.label("apply_velocity"))
                    .with_system(
                        reset_velocity
                            .label("reset_velocity")
                            .after("apply_velocity"),
                    )
                    .with_system(
                        play_animations
                            .label("play_animations")
                            .after("apply_velocity")
                            .before("reset_velocity"),
                    ),
            );
    }
}

fn spawn_player(mut commands: Commands, scenes: Res<SceneAssets>, gltf: Res<Assets<Gltf>>) {
    let model = gltf
        .get(&scenes.character)
        .expect("Failed to load player model");

    let height = 1.0;
    let radius = 0.4;
    commands
        .spawn((
            PbrBundle {
                transform: Transform {
                    translation: Vec3::new(0., 5., 0.),
                    scale: Vec3::splat(0.5),
                    ..default()
                },
                ..default()
            },
            RigidBody::KinematicVelocityBased,
            Collider::capsule_y(height / 2., radius),
            KinematicCharacterController {
                // Don’t allow climbing slopes larger than n degrees.
                max_slope_climb_angle: 45.0_f32.to_radians() as Real,
                // Automatically slide down on slopes smaller than n degrees.
                min_slope_slide_angle: 30.0_f32.to_radians() as Real,
                // The character offset is set to n multiplied by the collider’s height.
                offset: CharacterLength::Absolute(0.01),
                // Snap to the ground if the vertical distance to the ground is smaller than n.
                snap_to_ground: Some(CharacterLength::Absolute(0.001)),
                filter_flags: QueryFilterFlags::EXCLUDE_SENSORS,
                ..default()
            },
            Player,
            Name::new("Player"),
            Grounded::default(),
            CharacterVelocity::default(),
            Jump::default(),
        ))
        .with_children(|parent| {
            parent.spawn((
                Collider::capsule_y(height / 2., radius),
                Sensor,
                PlayerSensor,
                ActiveCollisionTypes::all(),
                Name::new("Player Sensor"),
            ));
            parent.spawn((
                PlayerCamera,
                Camera3dBundle {
                    transform: Transform::from_xyz(10., 2., 0.),
                    ..default()
                },
                Name::new("Player Camera"),
            ));
            parent.spawn((
                SceneBundle {
                    scene: model.scenes[0].clone(),
                    transform: Transform {
                        translation: Vec3::new(0., -height, 0.),
                        scale: Vec3::splat(0.02),
                        ..default()
                    },
                    ..default()
                },
                PlayerModel,
                Name::new("Player Model"),
            ));
        });
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
            - f32::from(jump.time_since_start).min(JUMP_DURATION);
        let max_gravity = G * 5.;
        let min_gravity = G * 0.1;
        // min and max look swapped because gravity is negative
        let gravity = (G * dt).clamp(max_gravity, min_gravity);
        velocity.0.y += gravity;
    }
}

fn handle_jump(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<(&Grounded, &mut CharacterVelocity, &mut Jump), With<Player>>,
) {
    let dt = time.delta_seconds();
    let jump_requested = actions.jump;
    for (grounded, mut velocity, mut jump) in &mut player_query {
        let y_speed = 10.;
        if jump_requested && f32::from(grounded.time_since_last_grounded) < 0.00001 {
            jump.time_since_start.start();
            jump.state = JumpState::InProgress;
        } else {
            jump.time_since_start.update(dt);

            let jump_ended = f32::from(jump.time_since_start) >= JUMP_DURATION;
            if jump_ended {
                jump.state = JumpState::Done;
            }
        }
        if matches!(jump.state, JumpState::InProgress) {
            velocity.0.y += jump.speed_fraction() * y_speed * dt
        }
    }
}

fn handle_horizontal_movement(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<(&mut CharacterVelocity,), With<Player>>,
    camera_query: Query<&Transform, With<PlayerCamera>>,
) {
    let dt = time.delta_seconds();
    let speed = 6.0;

    let camera = match camera_query.iter().next() {
        Some(transform) => transform,
        None => return,
    };
    let actions = match actions.player_movement {
        Some(actions) => actions,
        None => return,
    };

    let forward = (-camera.translation)
        .xz()
        .try_normalize()
        .unwrap_or(Vec2::Y);
    let sideward = forward.perp();
    let forward_action = forward * actions.y;
    let sideward_action = sideward * actions.x;
    let movement = (forward_action + sideward_action).normalize() * speed * dt;

    for (mut velocity,) in &mut player_query {
        velocity.0.x += movement.x;
        velocity.0.z += movement.y;
    }
}

/// Treat `CharacterVelocity` as readonly after this system.
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
    }
}

fn reset_velocity(mut player_query: Query<&mut CharacterVelocity, With<Player>>) {
    for mut velocity in &mut player_query {
        velocity.0 = default();
    }
}

fn play_animations(
    mut animation_player: Query<&mut AnimationPlayer>,
    player_query: Query<(&CharacterVelocity, &Grounded, &AnimationEntityLink), With<Player>>,
    mut model_query: Query<&mut Transform, With<PlayerModel>>,
    animations: Res<AnimationAssets>,
) {
    let (velocity, grounded, animation_entity_link) = match player_query.iter().next() {
        Some(player) => player,
        _ => {
            return;
        }
    };
    let mut animation_player = match animation_player.get_mut(animation_entity_link.0) {
        Ok(player) => player,
        _ => {
            error!("No animation player found for player character");
            return;
        }
    };

    let horizontal_velocity = Vec3 {
        y: 0.,
        ..velocity.0
    };
    let is_in_air = f32::from(grounded.time_since_last_grounded) > 1e-4;
    let has_horizontal_movement = horizontal_velocity.length() > 1e-4;

    if is_in_air {
        animation_player
            .play(animations.character_running.clone_weak())
            .repeat();
    } else if has_horizontal_movement {
        animation_player
            .play(animations.character_walking.clone_weak())
            .repeat();
    } else {
        animation_player
            .play(animations.character_idle.clone_weak())
            .repeat();
    }

    if has_horizontal_movement {
        for mut model in &mut model_query {
            model.rotation = look_at(horizontal_velocity.normalize(), Vect::Y);
        }
    }
}

#[derive(Component)]
pub struct AnimationEntityLink(pub Entity);

/// Source: <https://github.com/bevyengine/bevy/discussions/5564>
fn get_top_parent(mut curr_entity: Entity, parent_query: &Query<&Parent>) -> Entity {
    loop {
        if let Ok(parent) = parent_query.get(curr_entity) {
            curr_entity = parent.get();
        } else {
            break;
        }
    }
    curr_entity
}

pub fn link_animations(
    player_query: Query<Entity, Added<AnimationPlayer>>,
    parent_query: Query<&Parent>,
    animations_entity_link_query: Query<&AnimationEntityLink>,
    mut commands: Commands,
) {
    for entity in player_query.iter() {
        let top_entity = get_top_parent(entity, &parent_query);

        if animations_entity_link_query.get(top_entity).is_ok() {
            warn!("Multiple `AnimationPlayer`s are ambiguous for the same top parent");
        } else {
            commands
                .entity(top_entity)
                .insert(AnimationEntityLink(entity.clone()));
        }
    }
}
