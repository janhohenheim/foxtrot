use crate::spawning::{GameObject, PrimedGameObjectSpawner};
use bevy::ecs::system::EntityCommands;
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub const PATH: &str = "scenes/wallWoodDoorwayRound.glb";

pub fn load_scene(asset_server: &Res<AssetServer>) -> Handle<Gltf> {
    asset_server.load(PATH)
}

impl<'w, 's, 'a, 'b> PrimedGameObjectSpawner<'w, 's, 'a, 'b> {
    pub fn spawn_doorway(&'a mut self) -> EntityCommands<'w, 's, 'a> {
        let gltf = self
            .gltf
            .get(&self.handles.scenes[&GameObject::Doorway])
            .expect("Failed to load doorway scene");
        let mut entity_commands = self.commands.spawn(SceneBundle {
            scene: gltf.scenes[0].clone(),
            transform: Transform::from_scale(Vec3::splat(3.)),
            ..default()
        });
        entity_commands.with_children(|parent| {
            let offset = 0.002;
            parent.spawn((
                TransformBundle::from_transform(Transform::from_xyz(
                    -0.45,
                    0.5,
                    0.5 / 3. * 2. + 5. * offset,
                )),
                Collider::cuboid(0.04, 0.5, 0.5 / 3. + offset),
            ));
            parent.spawn((
                TransformBundle::from_transform(Transform::from_xyz(
                    -0.45,
                    0.5,
                    -0.5 / 3. * 2. - 5. * offset,
                )),
                Collider::cuboid(0.04, 0.5, 0.5 / 3. + offset),
            ));
        });
        entity_commands
    }
}
