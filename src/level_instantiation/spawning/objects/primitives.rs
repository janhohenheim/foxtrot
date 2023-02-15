use crate::level_instantiation::spawning::{
    GameObject, PrimedGameObjectSpawner, PrimedGameObjectSpawnerImplementor,
};
use anyhow::Result;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct EmptySpawner;

impl PrimedGameObjectSpawnerImplementor for EmptySpawner {
    fn spawn<'a, 'b: 'a>(
        &self,
        spawner: &'b mut PrimedGameObjectSpawner<'_, '_, 'a>,
        _object: GameObject,
        _transform: Transform,
    ) -> Result<Entity> {
        Ok(spawner.commands.spawn_empty().id())
    }
}

pub struct BoxSpawner;

impl PrimedGameObjectSpawnerImplementor for BoxSpawner {
    fn spawn<'a, 'b: 'a>(
        &self,
        spawner: &'b mut PrimedGameObjectSpawner<'_, '_, 'a>,
        _object: GameObject,
        transform: Transform,
    ) -> Result<Entity> {
        Ok(spawner
            .commands
            .spawn((
                TransformBundle::from_transform(transform),
                Collider::cuboid(1., 1., 1.),
                Name::new("Box Collider"),
            ))
            .id())
    }
}

pub struct SphereSpawner;

impl PrimedGameObjectSpawnerImplementor for SphereSpawner {
    fn spawn<'a, 'b: 'a>(
        &self,
        spawner: &'b mut PrimedGameObjectSpawner<'_, '_, 'a>,
        _object: GameObject,
        transform: Transform,
    ) -> Result<Entity> {
        Ok(spawner
            .commands
            .spawn((
                TransformBundle::from_transform(transform),
                Collider::ball(1.),
                Name::new("Sphere Collider"),
            ))
            .id())
    }
}

pub struct CapsuleSpawner;

impl PrimedGameObjectSpawnerImplementor for CapsuleSpawner {
    fn spawn<'a, 'b: 'a>(
        &self,
        spawner: &'b mut PrimedGameObjectSpawner<'_, '_, 'a>,
        _object: GameObject,
        transform: Transform,
    ) -> Result<Entity> {
        Ok(spawner
            .commands
            .spawn((
                TransformBundle::from_transform(transform),
                Collider::capsule_y(1., 1.),
                Name::new("Capsule Collider"),
            ))
            .id())
    }
}

pub struct TriangleSpawner;

impl PrimedGameObjectSpawnerImplementor for TriangleSpawner {
    fn spawn<'a, 'b: 'a>(
        &self,
        spawner: &'b mut PrimedGameObjectSpawner<'_, '_, 'a>,
        _object: GameObject,
        transform: Transform,
    ) -> Result<Entity> {
        Ok(spawner
            .commands
            .spawn((
                TransformBundle::from_transform(transform),
                Collider::triangle(Vect::ZERO, Vect::Y, Vect::X),
                Name::new("Triangle Collider"),
            ))
            .id())
    }
}
