use crate::level_instantiation::spawning::GameObject;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub(crate) fn spawn_empty(In(_transform): In<Transform>, mut commands: Commands) {
    commands.spawn(GameObject::Empty);
}

pub(crate) fn spawn_box(In(transform): In<Transform>, mut commands: Commands) {
    commands.spawn((
        TransformBundle::from_transform(transform),
        Collider::cuboid(1., 1., 1.),
        Name::new("Box Collider"),
        GameObject::Box,
    ));
}

pub(crate) fn spawn_sphere(In(transform): In<Transform>, mut commands: Commands) {
    commands.spawn((
        TransformBundle::from_transform(transform),
        Collider::ball(1.),
        Name::new("Sphere Collider"),
        GameObject::Sphere,
    ));
}

pub(crate) fn spawn_capsule(In(transform): In<Transform>, mut commands: Commands) {
    commands.spawn((
        TransformBundle::from_transform(transform),
        Collider::capsule_y(1., 1.),
        Name::new("Capsule Collider"),
        GameObject::Capsule,
    ));
}

pub(crate) fn spawn_triangle(In(transform): In<Transform>, mut commands: Commands) {
    commands.spawn((
        TransformBundle::from_transform(transform),
        Collider::triangle(Vect::ZERO, Vect::Y, Vect::X),
        Name::new("Triangle Collider"),
        GameObject::Triangle,
    ));
}
