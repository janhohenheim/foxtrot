use crate::level_instanciation::spawning::PrimedGameObjectSpawner;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

impl<'w, 's, 'a, 'b> PrimedGameObjectSpawner<'w, 's, 'a, 'b> {
    pub fn spawn_empty(&'a mut self) {}

    pub fn spawn_box(&'a mut self) {
        self.commands.spawn((
            TransformBundle::default(),
            Collider::cuboid(1., 1., 1.),
            Name::new("Box Collider"),
        ));
    }

    pub fn spawn_sphere(&'a mut self) {
        self.commands.spawn((
            TransformBundle::default(),
            Collider::ball(1.),
            Name::new("Sphere Collider"),
        ));
    }

    pub fn spawn_capsule(&'a mut self) {
        self.commands.spawn((
            TransformBundle::default(),
            Collider::capsule_y(1., 1.),
            Name::new("Capsule Collider"),
        ));
    }

    pub fn spawn_triangle(&'a mut self) {
        self.commands.spawn((
            TransformBundle::default(),
            Collider::triangle(Vect::ZERO, Vect::Y, Vect::X),
            Name::new("Triangle Collider"),
        ));
    }
}
