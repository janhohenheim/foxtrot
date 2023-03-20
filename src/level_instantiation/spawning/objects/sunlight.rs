use crate::level_instantiation::spawning::GameObject;

use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;

pub(crate) fn spawn(In(transform): In<Transform>, mut commands: Commands) {
    // directional 'sun' light
    commands.spawn((
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                ..default()
            },
            cascade_shadow_config: CascadeShadowConfigBuilder {
                first_cascade_far_bound: 7.0,
                maximum_distance: 100.0,
                ..default()
            }
            .into(),
            transform,
            ..default()
        },
        Name::new("Light"),
        GameObject::Sunlight,
    ));
}
