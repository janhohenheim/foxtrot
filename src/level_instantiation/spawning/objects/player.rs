use crate::{
    level_instantiation::spawning::objects::CollisionLayer,
    movement::character_controller::{CharacterAnimations, CharacterControllerBundle},
    particles,
    player_control::{
        actions::{
            create_player_action_input_manager_bundle, create_ui_action_input_manager_bundle,
        },
        player_embodiment::Player,
    },
};
use bevy::prelude::*;
use bevy_hanabi::EffectAsset;

pub(crate) const HEIGHT: f32 = 0.4;
pub(crate) const RADIUS: f32 = 0.3;

pub(crate) fn spawn(
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
                // Use the same names as in Blender
                CharacterAnimations {
                    idle: "Idle".into(),
                    walk: "Walk".into(),
                    aerial: "Run".into(),
                },
                create_player_action_input_manager_bundle(),
                create_ui_action_input_manager_bundle(),
            ))
            .with_children(|parent| {
                let particle_bundle = particles::create_sprint_particle_bundle(&mut effects);
                parent.spawn(particle_bundle);
            });
    }
}
