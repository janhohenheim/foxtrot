use crate::level_instantiation::spawning::GameObject;

use bevy::prelude::*;

pub(crate) fn spawn(In(transform): In<Transform>, mut commands: Commands) {
    commands.spawn((
        PointLightBundle {
            point_light: PointLight {
                color: Color::WHITE,
                intensity: 1.0,
                range: 1.0,
                radius: 1.0,
                shadows_enabled: true,
                ..default()
            },
            transform,
            ..default()
        },
        Name::new("Light"),
        GameObject::PointLight,
    ));
}
