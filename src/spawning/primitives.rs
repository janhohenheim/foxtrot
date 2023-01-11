use crate::spawning::PrimedGameObjectSpawner;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

impl<'w, 's, 'a, 'b> PrimedGameObjectSpawner<'w, 's, 'a, 'b> {
    pub fn spawn_empty(&'a mut self) {}

    pub fn spawn_box(&'a mut self) {
        self.commands.spawn((
            TransformBundle::default(),
            Collider::cuboid(3., 3., 3.),
            Name::new("Box"),
        ));
    }

    pub fn spawn_sphere(&'a mut self) {
        self.commands.spawn((
            TransformBundle::default(),
            Collider::ball(3.),
            Name::new("Sphere"),
        ));
    }

    pub fn spawn_capsule(&'a mut self) {
        self.commands.spawn((
            TransformBundle::default(),
            Collider::capsule_y(3., 1.),
            Name::new("Capsule"),
        ));
    }
}
