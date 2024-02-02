use crate::file_system_interaction::asset_loading::AnimationAssets;
use crate::level_instantiation::spawning::objects::CollisionLayer;
use crate::movement::general_movement::{CharacterAnimations, CharacterControllerBundle};
use crate::player_control::actions::{
    create_player_action_input_manager_bundle, create_ui_action_input_manager_bundle,
};
use crate::player_control::player_embodiment::Player;
use bevy::prelude::*;

pub(crate) const HEIGHT: f32 = 0.4;
pub(crate) const RADIUS: f32 = 0.3;

pub(crate) fn spawn(
    player: Query<Entity, Added<Player>>,
    mut commands: Commands,
    animations: Res<AnimationAssets>,
) {
    for entity in player.iter() {
        let mut controller = CharacterControllerBundle::capsule(HEIGHT, RADIUS);
        controller.collision_layers = controller
            .collision_layers
            .add_group(CollisionLayer::Player);

        commands.entity(entity).insert((
            PbrBundle::default(),
            Player,
            Name::new("Player"),
            controller,
            CharacterAnimations {
                idle: animations.character_idle.clone(),
                walk: animations.character_walking.clone(),
                aerial: animations.character_running.clone(),
            },
            create_player_action_input_manager_bundle(),
            create_ui_action_input_manager_bundle(),
        ));
    }
}
