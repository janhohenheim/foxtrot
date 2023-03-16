use crate::level_instantiation::spawning::{
    GameObject, PrimedGameObjectSpawner, PrimedGameObjectSpawnerImplementor,
};
use crate::player_control::actions::create_camera_action_input_manager_bundle;
use crate::player_control::camera::IngameCamera;
use anyhow::Result;
use bevy::prelude::*;
use bevy_dolly::prelude::*;

pub struct CameraSpawner;

impl PrimedGameObjectSpawnerImplementor for CameraSpawner {
    fn spawn<'a, 'b: 'a>(
        &self,
        spawner: &'b mut PrimedGameObjectSpawner<'_, '_, 'a>,
        _object: GameObject,
        transform: Transform,
    ) -> Result<Entity> {
        Ok(spawner
            .commands
            .spawn((
                IngameCamera::default(),
                Camera3dBundle {
                    transform,
                    ..default()
                },
                Rig::builder()
                    .with(Position::new(Vec3::ZERO))
                    .with(YawPitch::new().yaw_degrees(45.0).pitch_degrees(-30.0))
                    .with(Smooth::new_position_rotation(0.3, 0.3))
                    .with(Arm::new(Vec3::Z * 5.))
                    .with(
                        LookAt::new(Vec3::ZERO)
                            .tracking_smoothness(0.1)
                            .tracking_predictive(true),
                    )
                    .build(),
                create_camera_action_input_manager_bundle(),
                Name::new("Main Camera"),
            ))
            .id())
    }
}
