use crate::level_instanciation::spawning::{
    GameObject, PrimedGameObjectSpawner, PrimedGameObjectSpawnerImplementor,
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct EmptySpawner;

impl PrimedGameObjectSpawnerImplementor for EmptySpawner {
    fn spawn<'a, 'b: 'a>(
        &self,
        _spawner: &'b mut PrimedGameObjectSpawner<'_, '_, 'a, '_>,
        _object: GameObject,
    ) {
    }
}

pub struct BoxSpawner;

impl PrimedGameObjectSpawnerImplementor for BoxSpawner {
    fn spawn<'a, 'b: 'a>(
        &self,
        spawner: &'b mut PrimedGameObjectSpawner<'_, '_, 'a, '_>,
        _object: GameObject,
    ) {
        spawner.commands.spawn((
            TransformBundle::default(),
            Collider::cuboid(1., 1., 1.),
            Name::new("Box Collider"),
        ));
    }
}

pub struct SphereSpawner;

impl PrimedGameObjectSpawnerImplementor for SphereSpawner {
    fn spawn<'a, 'b: 'a>(
        &self,
        spawner: &'b mut PrimedGameObjectSpawner<'_, '_, 'a, '_>,
        _object: GameObject,
    ) {
        spawner.commands.spawn((
            TransformBundle::default(),
            Collider::ball(1.),
            Name::new("Sphere Collider"),
        ));
    }
}

pub struct CapsuleSpawner;

impl PrimedGameObjectSpawnerImplementor for CapsuleSpawner {
    fn spawn<'a, 'b: 'a>(
        &self,
        spawner: &'b mut PrimedGameObjectSpawner<'_, '_, 'a, '_>,
        _object: GameObject,
    ) {
        spawner.commands.spawn((
            TransformBundle::default(),
            Collider::capsule_y(1., 1.),
            Name::new("Capsule Collider"),
        ));
    }
}

pub struct TriangleSpawner;

impl PrimedGameObjectSpawnerImplementor for TriangleSpawner {
    fn spawn<'a, 'b: 'a>(
        &self,
        spawner: &'b mut PrimedGameObjectSpawner<'_, '_, 'a, '_>,
        _object: GameObject,
    ) {
        spawner.commands.spawn((
            TransformBundle::default(),
            Collider::triangle(Vect::ZERO, Vect::Y, Vect::X),
            Name::new("Triangle Collider"),
        ));
    }
}
