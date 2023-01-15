use crate::dialog::{DialogId, DialogTarget};
use crate::spawning::{GameObject, PrimedGameObjectSpawner};
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

/// Source: <https://opengameart.org/content/fox-and-shiba>
pub const PATH: &str = "scenes/Fox.glb";

pub fn load_scene(asset_server: &Res<AssetServer>) -> Handle<Gltf> {
    asset_server.load(PATH)
}

impl<'w, 's, 'a, 'b> PrimedGameObjectSpawner<'w, 's, 'a, 'b> {
    pub fn spawn_npc(&'a mut self) {
        let gltf = self
            .gltf
            .get(&self.handles.scenes[&GameObject::Npc])
            .unwrap_or_else(|| panic!("Failed to load scene from {PATH}"));
        let height = 1.0;
        let radius = 0.4;
        self.commands
            .spawn((
                PbrBundle::default(),
                Name::new("NPC"),
                RigidBody::Fixed,
                Collider::capsule_y(height / 2., radius),
            ))
            .with_children(|parent| {
                parent.spawn((
                    DialogTarget {
                        dialog_id: DialogId::new("sample"),
                    },
                    Name::new("NPC Dialog Collider"),
                    Collider::cylinder(height / 2., radius * 5.),
                    Sensor,
                    ActiveEvents::COLLISION_EVENTS,
                    ActiveCollisionTypes::KINEMATIC_STATIC,
                ));
                parent.spawn((
                    SceneBundle {
                        scene: gltf.scenes[0].clone(),
                        transform: Transform {
                            translation: Vec3::new(0., -height, 0.),
                            scale: Vec3::splat(0.02),
                            ..default()
                        },
                        ..default()
                    },
                    Name::new("NPC Model"),
                ));
            });
    }
}
