use crate::{
    file_system_interaction::asset_loading::GltfAssets,
    player_control::{actions::create_camera_action_input_manager_bundle, camera::IngameCamera},
    GameState,
};
use bevy::{gltf::Gltf, prelude::*};
use bevy_atmosphere::prelude::*;
use bevy_dolly::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Playing), spawn_level);
}

fn spawn_level(mut commands: Commands, models: Res<Assets<Gltf>>, gltf_assets: Res<GltfAssets>) {
    let gltf = models.get(&gltf_assets.level).unwrap();
    commands.spawn((
        SceneBundle {
            scene: gltf.scenes[0].clone(),
            ..default()
        },
        Name::new("Level"),
    ));

    commands.spawn((
        Name::new("Camera"),
        Camera3dBundle::default(),
        IngameCamera::default(),
        AtmosphereCamera::default(),
        IsDefaultUiCamera,
        Rig::builder()
            .with(Position::default())
            .with(YawPitch::default())
            .with(Smooth::default())
            .with(Arm::new(default()))
            .with(LookAt::new(default()).tracking_predictive(true))
            .build(),
        create_camera_action_input_manager_bundle(),
    ));
}
