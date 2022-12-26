use crate::actions::Actions;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Grounded {
    grounded: bool,
}

impl Default for Grounded {
    fn default() -> Self {
        Self { grounded: false }
    }
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
                    .with_system(check_ground.after("add_gravity")),
            );
    }
}

fn spawn_player(mut commands: Commands, textures: Res<TextureAssets>) {
    commands.spawn((
        RigidBody::KinematicPositionBased,
        Collider::ball(0.5),
        KinematicCharacterController::default(),
        Player,
        Grounded::default(),
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
        let gravity = Vec2::new(0.0, -9.81 * 100.0 * time.delta_seconds());
        controller.translation = Some(
            controller
                .translation
                .map(|translation| translation + gravity)
                .unwrap_or(gravity),
        );
    }
}

fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    mut player_query: Query<(&Grounded, &mut KinematicCharacterController), With<Player>>,
) {
    if actions.player_movement.is_none() {
        return;
    }
    let vertical_speed = 300.0;
    let jump_speed = 1_000.0;
    for (grounded, mut controller) in &mut player_query {
        let movement = Vec2::new(
            actions.player_movement.unwrap().x * vertical_speed * time.delta_seconds(),
            if grounded.grounded {
                actions.player_movement.unwrap().y * jump_speed * time.delta_seconds()
            } else {
                0.0
            },
        );
        controller.translation = Some(movement);
    }
}
