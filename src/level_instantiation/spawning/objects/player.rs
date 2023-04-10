use crate::file_system_interaction::asset_loading::{AnimationAssets, SceneAssets};
use crate::level_instantiation::spawning::objects::GameCollisionGroup;
use crate::level_instantiation::spawning::GameObject;
use crate::movement::general_movement::{CharacterAnimations, CharacterControllerBundle, Model};
use crate::player_control::actions::{
    create_player_action_input_manager_bundle, create_ui_action_input_manager_bundle,
};
use crate::player_control::player_embodiment::Player;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::f32::consts::TAU;

pub(crate) const HEIGHT: f32 = 0.4;
pub(crate) const RADIUS: f32 = 0.3;

pub(crate) fn spawn(
    In(transform): In<Transform>,
    mut commands: Commands,
    animations: Res<AnimationAssets>,
    scene_handles: Res<SceneAssets>,
) {
    let entity = commands
        .spawn((
            PbrBundle {
                transform,
                ..default()
            },
            Player,
            Name::new("Player"),
            Ccd::enabled(),
            CharacterControllerBundle::capsule(HEIGHT, RADIUS),
            CharacterAnimations {
                idle: animations.character_idle.clone(),
                walk: animations.character_walking.clone(),
                aerial: animations.character_running.clone(),
            },
            CollisionGroups::new(
                GameCollisionGroup::PLAYER.into(),
                GameCollisionGroup::ALL.into(),
            ),
            create_player_action_input_manager_bundle(),
            create_ui_action_input_manager_bundle(),
            GameObject::Player,
        ))
        .id();

    commands
        .spawn((
            Model { target: entity },
            SpatialBundle::default(),
            Name::new("Player Model Parent"),
        ))
        .with_children(|parent| {
            parent.spawn((
                SceneBundle {
                    scene: scene_handles.character.clone(),
                    transform: Transform {
                        translation: Vec3::new(0., -HEIGHT / 2. - RADIUS, 0.),
                        rotation: Quat::from_rotation_y(TAU / 2.),
                        scale: Vec3::splat(0.01),
                    },
                    ..default()
                },
                Name::new("Player Model"),
            ));
        });
}
