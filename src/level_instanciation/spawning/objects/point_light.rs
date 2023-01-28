use crate::level_instanciation::spawning::PrimedGameObjectSpawner;
use bevy::prelude::*;

impl<'w, 's, 'a, 'b> PrimedGameObjectSpawner<'w, 's, 'a, 'b> {
    pub fn spawn_point_light(&'a mut self) {
        self.commands.spawn((
            PointLightBundle {
                point_light: PointLight {
                    color: Color::WHITE,
                    intensity: 1.0,
                    range: 1.0,
                    radius: 1.0,
                    shadows_enabled: true,
                    ..default()
                },
                ..default()
            },
            Name::new("Light"),
        ));
    }
}
