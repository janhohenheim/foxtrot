use crate::level_instanciation::spawning::{GameObject, PrimedGameObjectSpawner};
use bevy::pbr::NotShadowCaster;
use bevy::prelude::*;

pub fn load_mesh(meshes: &mut ResMut<Assets<Mesh>>) -> Handle<Mesh> {
    meshes.add(Mesh::from(shape::UVSphere {
        radius: 1.0,
        ..default()
    }))
}

impl<'w, 's, 'a, 'b> PrimedGameObjectSpawner<'w, 's, 'a, 'b> {
    pub fn spawn_orb(&'a mut self) {
        self.commands
            .spawn((
                MaterialMeshBundle {
                    mesh: self.handles.meshes[&GameObject::Orb].clone(),
                    material: self.materials.glowy.clone(),
                    transform: Transform::from_translation((0., 1.5, 0.).into()),
                    ..default()
                },
                Name::new("Orb"),
                NotShadowCaster,
            ))
            .with_children(|parent| {
                parent.spawn((PointLightBundle {
                    point_light: PointLight {
                        intensity: 10_000.,
                        radius: 1.,
                        color: Color::rgb(0.5, 0.1, 0.),
                        shadows_enabled: true,
                        ..default()
                    },
                    ..default()
                },));
            });
    }
}
