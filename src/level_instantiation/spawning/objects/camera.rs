use crate::level_instantiation::spawning::{
    GameObject, PrimedGameObjectSpawner, PrimedGameObjectSpawnerImplementor,
};
use crate::player_control::actions::create_camera_action_input_manager_bundle;
use crate::player_control::camera::IngameCamera;
use anyhow::Result;
use bevy::prelude::*;

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
                create_camera_action_input_manager_bundle(),
                Name::new("Main Camera"),
            ))
            .id())
    }
}
