use crate::file_system_interaction::asset_loading::{AnimationAssets, SceneAssets};
use crate::level_instanciation::spawning::animation_link::link_animations;
use crate::level_instanciation::spawning::change_parent::change_parent;
use crate::level_instanciation::spawning::counter::Counter;
use crate::level_instanciation::spawning::duplication::duplicate;
use crate::level_instanciation::spawning::objects::level::LevelSpawner;
use crate::level_instanciation::spawning::objects::npc::NpcSpawner;
use crate::level_instanciation::spawning::objects::orb::OrbSpawner;
use crate::level_instanciation::spawning::objects::player::PlayerSpawner;
use crate::level_instanciation::spawning::objects::point_light::PointLightSpawner;
use crate::level_instanciation::spawning::objects::primitives::{
    BoxSpawner, CapsuleSpawner, EmptySpawner, SphereSpawner, TriangleSpawner,
};
use crate::level_instanciation::spawning::objects::sunlight::SunlightSpawner;
use crate::level_instanciation::spawning::post_spawn_modification::{
    read_colliders, set_hidden, set_texture_to_repeat,
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
                    .with_system(set_texture_to_repeat)
                    .with_system(set_hidden),
            );
    }
}

impl<'w, 's, 'a, 'b> PrimedGameObjectSpawner<'w, 's, 'a, 'b> {
    pub fn new(
        outer_spawner: &'a GameObjectSpawner,
        commands: &'a mut ChildBuilder<'w, 's, 'b>,
        gltf: &'a Res<'a, Assets<Gltf>>,
        materials: &'a Res<'a, Materials>,
        animations: &'a Res<'a, AnimationAssets>,
        scenes: &'a Res<'a, SceneAssets>,
    ) -> Self {
        Self {
            outer_spawner,
            commands,
            gltf,
            materials,
            animations,
            scenes,
        }
    }

    pub fn spawn(&mut self, object: GameObject) {
        self.outer_spawner.implementors[&object].spawn(self, object);
    }
}

fn load_assets_for_spawner(mut commands: Commands, mut mesh_assets: ResMut<Assets<Mesh>>) {
    let mut implementors = HashMap::new();

    implementors.insert(
        GameObject::Box,
        Box::new(BoxSpawner) as Box<dyn PrimedGameObjectSpawnerImplementor + Send + Sync>,
    );
    implementors.insert(GameObject::Orb, Box::new(OrbSpawner));
    implementors.insert(GameObject::Player, Box::new(PlayerSpawner));
    implementors.insert(GameObject::Sphere, Box::new(SphereSpawner));
    implementors.insert(GameObject::Capsule, Box::new(CapsuleSpawner));
    implementors.insert(GameObject::Npc, Box::new(NpcSpawner));
    implementors.insert(GameObject::Sunlight, Box::new(SunlightSpawner));
    implementors.insert(GameObject::PointLight, Box::new(PointLightSpawner));
    implementors.insert(GameObject::Triangle, Box::new(TriangleSpawner));
    implementors.insert(GameObject::Empty, Box::new(EmptySpawner));
    implementors.insert(GameObject::Level, Box::new(LevelSpawner));

    let mut meshes = HashMap::new();
    for (game_object, implementor) in implementors.iter() {
        if let Some(handle) = implementor.create_mesh(&mut mesh_assets) {
            meshes.insert(*game_object, handle);
        }
    }

    commands.insert_resource(GameObjectSpawner {
        meshes,
        implementors,
    });
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

pub trait PrimedGameObjectSpawnerImplementor {
    fn create_mesh(&self, _mesh_assets: &mut ResMut<Assets<Mesh>>) -> Option<Handle<Mesh>> {
        None
    }
    fn spawn(&self, spawner: &mut PrimedGameObjectSpawner, object: GameObject);
}

#[derive(Resource)]
pub struct GameObjectSpawner {
    meshes: HashMap<GameObject, Handle<Mesh>>,
    implementors: HashMap<GameObject, Box<dyn PrimedGameObjectSpawnerImplementor + Send + Sync>>,
}

#[derive(Resource)]
pub struct PrimedGameObjectSpawner<'w, 's, 'a, 'b> {
    pub outer_spawner: &'a GameObjectSpawner,
    pub gltf: &'a Res<'a, Assets<Gltf>>,
    pub materials: &'a Res<'a, Materials>,
    pub commands: &'a mut ChildBuilder<'w, 's, 'b>,
    pub animations: &'a Res<'a, AnimationAssets>,
    pub scenes: &'a Res<'a, SceneAssets>,
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
        scenes: &'a Res<'a, SceneAssets>,
    ) -> PrimedGameObjectSpawner<'w, 's, 'a, 'b> {
        PrimedGameObjectSpawner::new(self, commands, gltf, materials, animations, scenes)
    }
}
