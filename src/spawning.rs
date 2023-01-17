use crate::spawning::animation_link::link_animations;
use crate::spawning::change_parent::change_parent;
use crate::spawning::counter::Counter;
use crate::spawning::duplication::duplicate;
use crate::spawning::objects::*;
use crate::spawning::read_colliders::read_colliders;
use crate::spawning::spawn::{spawn_delayed, spawn_requested, DelayedSpawnEvents};
use crate::spawning::spawn_container::{sync_container_registry, SpawnContainerRegistry};
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
mod read_colliders;
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
            .add_startup_system(load_assets_for_spawner)
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(spawn_requested.label("spawn_requested"))
                    .with_system(spawn_delayed)
                    .with_system(sync_container_registry.before("spawn_requested"))
                    .with_system(change_parent.after("spawn_requested"))
                    .with_system(duplicate.after("spawn_requested"))
                    .with_system(link_animations.after("spawn_requested"))
                    .with_system(read_colliders),
            );
    }
}

impl<'w, 's, 'a, 'b> PrimedGameObjectSpawner<'w, 's, 'a, 'b> {
    pub fn spawn(&'a mut self, object: &GameObject) {
        match *object {
            GameObject::Grass => self.spawn_grass(),
            GameObject::Doorway => self.spawn_doorway(),
            GameObject::Wall => self.spawn_wall(),
            GameObject::Roof => self.spawn_roof(),
            GameObject::RoofRight => self.spawn_roof_right(),
            GameObject::RoofLeft => self.spawn_roof_left(),
            GameObject::Sunlight => self.spawn_sunlight(),
            GameObject::Npc => self.spawn_npc(),
            GameObject::Empty => self.spawn_empty(),
            GameObject::Box => self.spawn_box(),
            GameObject::Sphere => self.spawn_sphere(),
            GameObject::Capsule => self.spawn_capsule(),
            GameObject::Triangle => self.spawn_triangle(),
            GameObject::PointLight => self.spawn_point_light(),
            GameObject::Player => self.spawn_player(),
            GameObject::House => self.spawn_house(),
        };
    }
}

fn load_assets_for_spawner(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
) {
    let mut meshes = HashMap::new();
    meshes.insert(GameObject::Grass, grass::create_mesh(&mut mesh_assets));

    let mut materials = HashMap::new();
    materials.insert(
        GameObject::Grass,
        grass::load_material(&asset_server, &mut material_assets),
    );

    let mut scenes = HashMap::new();
    scenes.insert(GameObject::Doorway, doorway::load_scene(&asset_server));
    scenes.insert(GameObject::Wall, wall::load_scene(&asset_server));
    scenes.insert(GameObject::Roof, roof::load_scene(&asset_server));
    scenes.insert(GameObject::RoofRight, roof_right::load_scene(&asset_server));
    scenes.insert(GameObject::RoofLeft, roof_left::load_scene(&asset_server));
    scenes.insert(GameObject::Npc, npc::load_scene(&asset_server));
    scenes.insert(GameObject::House, house::load_scene(&asset_server));

    commands.insert_resource(GameObjectSpawner {
        meshes,
        materials,
        scenes,
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
    Grass,
    Doorway,
    Wall,
    Roof,
    RoofRight,
    RoofLeft,
    Sunlight,
    PointLight,
    Npc,
    Player,
    House,
}

impl Default for GameObject {
    fn default() -> Self {
        Self::Empty
    }
}

#[derive(Resource)]
pub struct GameObjectSpawner {
    meshes: HashMap<GameObject, Handle<Mesh>>,
    materials: HashMap<GameObject, Handle<StandardMaterial>>,
    scenes: HashMap<GameObject, Handle<Gltf>>,
}

#[derive(Resource)]
pub struct PrimedGameObjectSpawner<'w, 's, 'a, 'b> {
    handles: &'a GameObjectSpawner,
    gltf: &'a Res<'a, Assets<Gltf>>,
    commands: &'a mut ChildBuilder<'w, 's, 'b>,
}

impl<'a, 'b, 'c, 'w, 's> GameObjectSpawner
where
    'c: 'a,
{
    pub fn attach(
        &'c self,
        commands: &'a mut ChildBuilder<'w, 's, 'b>,
        gltf: &'a Res<'a, Assets<Gltf>>,
    ) -> PrimedGameObjectSpawner<'w, 's, 'a, 'b> {
        PrimedGameObjectSpawner {
            handles: self,
            commands,
            gltf,
        }
    }
}
