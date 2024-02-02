use bevy::pbr::CascadeShadowConfigBuilder;
use bevy::prelude::*;

#[derive(Debug, Default, Clone, Copy, Eq, PartialEq, Reflect, Component)]
#[reflect(Component)]
pub(crate) struct Sun;

pub(crate) fn spawn(sun: Query<Entity, Added<Sun>>, mut commands: Commands) {
    for entity in sun.iter() {
        commands
            .entity(entity)
            .insert((
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
                    ..default()
                },
                Name::new("Sun"),
            ))
            .remove::<DirectionalLight>();
    }
}
