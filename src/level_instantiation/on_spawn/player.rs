use crate::{
    movement::{character_controller::CharacterControllerBundle, physics::CollisionLayer},
    particles,
    player_control::actions::{
        create_player_action_input_manager_bundle, create_ui_action_input_manager_bundle,
    },
    GameState,
};
use bevy::prelude::*;
use bevy_hanabi::EffectAsset;
use serde::{Deserialize, Serialize};

pub(crate) const HEIGHT: f32 = 0.4;
pub(crate) const RADIUS: f32 = 0.3;

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
pub(crate) struct Player;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>()
        .add_systems(Update, spawn.run_if(in_state(GameState::Playing)));
}

fn spawn(
    player: Query<(Entity, &Transform), Added<Player>>,
    mut commands: Commands,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    for (entity, transform) in player.iter() {
        let mut controller = CharacterControllerBundle::capsule(HEIGHT, RADIUS, transform.scale.y);
        controller.collision_layers.memberships |= CollisionLayer::Player;

        commands
            .entity(entity)
            .insert((
                controller,
                create_player_action_input_manager_bundle(),
                create_ui_action_input_manager_bundle(),
            ))
            .with_children(|parent| {
                let particle_bundle = particles::create_sprint_particle_bundle(&mut effects);
                parent.spawn(particle_bundle);
            });
    }
}
