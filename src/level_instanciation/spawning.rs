use crate::file_system_interaction::asset_loading::AnimationAssets;
use crate::level_instanciation::spawning::animation_link::link_animations;
use crate::level_instanciation::spawning::change_parent::change_parent;
use crate::level_instanciation::spawning::counter::Counter;
use crate::level_instanciation::spawning::duplication::duplicate;
use crate::level_instanciation::spawning::objects::*;
use crate::level_instanciation::spawning::post_spawn_modification::{
    read_colliders, set_texture_to_repeat,
};
use crate::level_instanciation::spawning::spawn::{
    spawn_delayed, spawn_requested, DelayedSpawnEvents,
};
use crate::level_instanciation::spawning::spawn_container::{
    sync_container_registry, SpawnContainerRegistry,
};
use crate::shader::Materials;
use crate::GameState;
pub use animation_link::AnimationEntityLink;
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy::utils::HashMap;
pub use event::*;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

pub mod objects;
mod spawn_container;
pub struct SpawningPlugin;
mod animation_link;
mod change_parent;
mod counter;
mod duplication;
mod event;
mod post_spawn_modification;
mod spawn;

impl Plugin for SpawningPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnEvent>()
            .add_event::<ParentChangeEvent>()
            .add_event::<DuplicationEvent>()
            .add_event::<DelayedSpawnEvent>()
            .init_resource::<SpawnContainerRegistry>()
            .init_resource::<Counter>()
            .init_resource::<DelayedSpawnEvents>()
            .register_type::<DelayedSpawnEvent>()
            .register_type::<SpawnEvent>()
            .register_type::<ParentChangeEvent>()
            .register_type::<DuplicationEvent>()
            .register_type::<SpawnTracker>()
            .register_type::<SpawnContainerRegistry>()
            .register_type::<DelayedSpawnEvents>()
            .register_type::<Counter>()
            .register_type::<AnimationEntityLink>()
            .add_system_set(
                SystemSet::on_enter(GameState::Loading).with_system(load_assets_for_spawner),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(spawn_requested.label("spawn_requested"))
                    .with_system(spawn_delayed)
                    .with_system(sync_container_registry.before("spawn_requested"))
                    .with_system(change_parent.after("spawn_requested"))
                    .with_system(duplicate.after("spawn_requested"))
                    .with_system(link_animations.after("spawn_requested")),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(read_colliders)
                    .with_system(set_texture_to_repeat),
            );
    }
}

impl<'w, 's, 'a, 'b> PrimedGameObjectSpawner<'w, 's, 'a, 'b> {
    pub fn spawn(&'a mut self, object: &GameObject) {
        match *object {
            GameObject::Sunlight => self.spawn_sunlight(),
            GameObject::Npc => self.spawn_npc(),
            GameObject::Empty => self.spawn_empty(),
            GameObject::Box => self.spawn_box(),
            GameObject::Sphere => self.spawn_sphere(),
            GameObject::Capsule => self.spawn_capsule(),
            GameObject::Triangle => self.spawn_triangle(),
            GameObject::PointLight => self.spawn_point_light(),
            GameObject::Player => self.spawn_player(),
            GameObject::Level => self.spawn_level(),
            GameObject::Orb => self.spawn_orb(),
        };
    }
}

fn load_assets_for_spawner(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
) {
    let mut scenes = HashMap::new();
    scenes.insert(GameObject::Npc, npc::load_scene(&asset_server));
    scenes.insert(GameObject::Level, level::load_scene(&asset_server));

    let mut meshes = HashMap::new();
    meshes.insert(GameObject::Orb, orb::load_mesh(&mut mesh_assets));

    commands.insert_resource(GameObjectSpawner { meshes, scenes });
}

#[derive(Debug, Component, Clone, PartialEq, Default, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct SpawnTracker {
    pub object: GameObject,
}

impl SpawnTracker {
    pub fn get_default_name(&self) -> String {
        format!("{:?}", self.object)
    }
}

impl From<SpawnEvent> for SpawnTracker {
    fn from(value: SpawnEvent) -> Self {
        Self {
            object: value.object,
        }
    }
}

#[derive(
    Debug,
    EnumIter,
    Component,
    Clone,
    Copy,
    Eq,
    PartialEq,
    Hash,
    Reflect,
    FromReflect,
    Serialize,
    Deserialize,
)]
#[reflect(Component, Serialize, Deserialize)]
pub enum GameObject {
    Empty,
    Box,
    Triangle,
    Sphere,
    Capsule,
    Sunlight,
    PointLight,
    Npc,
    Player,
    Level,
    Orb,
}

impl Default for GameObject {
    fn default() -> Self {
        Self::Empty
    }
}

#[derive(Resource)]
pub struct GameObjectSpawner {
    scenes: HashMap<GameObject, Handle<Gltf>>,
    meshes: HashMap<GameObject, Handle<Mesh>>,
}

#[derive(Resource)]
pub struct PrimedGameObjectSpawner<'w, 's, 'a, 'b> {
    handles: &'a GameObjectSpawner,
    gltf: &'a Res<'a, Assets<Gltf>>,
    materials: &'a Res<'a, Materials>,
    commands: &'a mut ChildBuilder<'w, 's, 'b>,
    animations: &'a Res<'a, AnimationAssets>,
}

impl<'a, 'b, 'c, 'w, 's> GameObjectSpawner
where
    'c: 'a,
{
    pub fn attach(
        &'c self,
        commands: &'a mut ChildBuilder<'w, 's, 'b>,
        gltf: &'a Res<'a, Assets<Gltf>>,
        materials: &'a Res<'a, Materials>,
        animations: &'a Res<'a, AnimationAssets>,
    ) -> PrimedGameObjectSpawner<'w, 's, 'a, 'b> {
        PrimedGameObjectSpawner {
            handles: self,
            commands,
            gltf,
            materials,
            animations,
        }
    }
}
