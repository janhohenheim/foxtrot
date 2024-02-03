use crate::file_system_interaction::asset_loading::GltfAssets;
use crate::level_instantiation::spawning::objects::CollisionLayer;
use crate::movement::character_controller::{CharacterAnimations, CharacterControllerBundle};
use crate::particles;
use crate::player_control::actions::{
    create_player_action_input_manager_bundle, create_ui_action_input_manager_bundle,
};
use crate::player_control::player_embodiment::Player;
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy_hanabi::EffectAsset;

pub(crate) const HEIGHT: f32 = 0.4;
pub(crate) const RADIUS: f32 = 0.3;

pub(crate) fn spawn(
    player: Query<(Entity, &Transform), Added<Player>>,
    mut commands: Commands,
    gltf_assets: Res<GltfAssets>,
    gltfs: Res<Assets<Gltf>>,
    mut effects: ResMut<Assets<EffectAsset>>,
) {
    for (entity, transform) in player.iter() {
        let mut controller = CharacterControllerBundle::capsule(HEIGHT, RADIUS, transform.scale.y);
        controller.collision_layers = controller
            .collision_layers
            .add_group(CollisionLayer::Player);

        let level = gltfs.get(gltf_assets.level.clone()).unwrap();
        let animations = &level.named_animations;

        commands
            .entity(entity)
            .insert((
                controller,
                CharacterAnimations {
                    idle: animations["Idle"].clone(),
                    walk: animations["Walk"].clone(),
                    aerial: animations["Run"].clone(),
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
