use crate::level_instanciation::spawning::{
    GameObject, PrimedGameObjectSpawner, PrimedGameObjectSpawnerImplementor,
};
use crate::player_control::camera::IngameCamera;
use bevy::prelude::*;

pub struct CameraSpawner;

impl PrimedGameObjectSpawnerImplementor for CameraSpawner {
    fn spawn<'a, 'b: 'a>(
        &self,
        spawner: &'b mut PrimedGameObjectSpawner<'_, '_, 'a>,
        _object: GameObject,
        transform: Transform,
    ) -> Entity {
        spawner
            .commands
            .spawn((
                IngameCamera::default(),
                Camera3dBundle {
                    transform,
                    ..default()
                },
                Name::new("Main Camera"),
            ))
            .id()
    }
}
