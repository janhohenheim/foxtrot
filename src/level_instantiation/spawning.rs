use crate::file_system_interaction::asset_loading::{AnimationAssets, SceneAssets};
use crate::level_instantiation::spawning::animation_link::link_animations;
use crate::level_instantiation::spawning::objects::camera::CameraSpawner;
use crate::level_instantiation::spawning::objects::level::LevelSpawner;
use crate::level_instantiation::spawning::objects::npc::NpcSpawner;
use crate::level_instantiation::spawning::objects::orb::OrbSpawner;
use crate::level_instantiation::spawning::objects::player::PlayerSpawner;
use crate::level_instantiation::spawning::objects::point_light::PointLightSpawner;
use crate::level_instantiation::spawning::objects::primitives::{
    BoxSpawner, CapsuleSpawner, EmptySpawner, SphereSpawner, TriangleSpawner,
};
use crate::level_instantiation::spawning::objects::skydome::SkydomeSpawner;
use crate::level_instantiation::spawning::objects::sunlight::SunlightSpawner;
use crate::level_instantiation::spawning::post_spawn_modification::{
    despawn_removed, set_color, set_hidden, set_shadows,
};
use crate::level_instantiation::spawning::spawn::{
    despawn, spawn_delayed, spawn_requested, DelayedSpawnEvents, Despawn,
};
use crate::shader::Materials;
use crate::util::log_error::log_errors;
use crate::GameState;
pub use animation_link::AnimationEntityLink;
use anyhow::Result;
use bevy::prelude::*;
use bevy::utils::HashMap;
pub use event::*;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

pub mod objects;
pub struct SpawningPlugin;
mod animation_link;
mod event;
mod post_spawn_modification;
pub mod spawn;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
pub struct SpawnRequestedLabel;

impl Plugin for SpawningPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnEvent>();
        app.add_event::<DelayedSpawnEvent>()
            .init_resource::<DelayedSpawnEvents>()
            .register_type::<DelayedSpawnEvent>()
            .register_type::<SpawnEvent>()
            .register_type::<SpawnTracker>()
            .register_type::<Despawn>()
            .register_type::<DelayedSpawnEvents>()
            .register_type::<AnimationEntityLink>()
            .add_system_set(
                SystemSet::on_exit(GameState::Loading).with_system(load_assets_for_spawner),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(spawn_requested.pipe(log_errors).label(SpawnRequestedLabel))
                    .with_system(spawn_delayed)
                    .with_system(despawn)
                    .with_system(link_animations.after(SpawnRequestedLabel)),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(set_hidden)
                    .with_system(despawn_removed)
                    .with_system(set_color.pipe(log_errors))
                    .with_system(set_shadows.pipe(log_errors)),
            );
    }
}

impl<'w, 's, 'a> PrimedGameObjectSpawner<'w, 's, 'a> {
    pub fn spawn<'c: 'a>(&'c mut self, object: GameObject, transform: Transform) -> Result<Entity> {
        #[cfg(feature = "tracing")]
        let _span = info_span!("spawn").entered();
        self.outer_spawner.implementors[&object].spawn(self, object, transform)
    }
}

fn load_assets_for_spawner(mut commands: Commands, mut mesh_assets: ResMut<Assets<Mesh>>) {
    #[cfg(feature = "tracing")]
    let _span = info_span!("load_assets_for_spawner").entered();
    let mut implementors = HashMap::new();

    for game_object in GameObject::iter() {
        let implementor: Box<dyn PrimedGameObjectSpawnerImplementor + Send + Sync> =
            match game_object {
                GameObject::Box => Box::new(BoxSpawner),
                GameObject::Orb => Box::new(OrbSpawner),
                GameObject::Player => Box::new(PlayerSpawner),
                GameObject::Sphere => Box::new(SphereSpawner),
                GameObject::Capsule => Box::new(CapsuleSpawner),
                GameObject::Npc => Box::new(NpcSpawner),
                GameObject::Sunlight => Box::new(SunlightSpawner),
                GameObject::PointLight => Box::new(PointLightSpawner),
                GameObject::Triangle => Box::new(TriangleSpawner),
                GameObject::Empty => Box::new(EmptySpawner),
                GameObject::Camera => Box::new(CameraSpawner),
                GameObject::Level => Box::new(LevelSpawner),
                GameObject::Skydome => Box::new(SkydomeSpawner),
            };
        implementors.insert(game_object, implementor);
    }
    implementors.insert(
        GameObject::Box,
        Box::new(BoxSpawner) as Box<dyn PrimedGameObjectSpawnerImplementor + Send + Sync>,
    );

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
    Camera,
    Skydome,
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
    fn spawn<'a, 'b: 'a>(
        &self,
        spawner: &'b mut PrimedGameObjectSpawner<'_, '_, 'a>,
        object: GameObject,
        transform: Transform,
    ) -> Result<Entity>;
}

#[derive(Resource)]
pub struct GameObjectSpawner {
    meshes: HashMap<GameObject, Handle<Mesh>>,
    implementors: HashMap<GameObject, Box<dyn PrimedGameObjectSpawnerImplementor + Send + Sync>>,
}

#[derive(Resource)]
pub struct PrimedGameObjectSpawner<'w, 's, 'a> {
    pub outer_spawner: &'a GameObjectSpawner,
    pub scenes: &'a Res<'a, Assets<Scene>>,
    pub materials: &'a Res<'a, Materials>,
    pub commands: &'a mut Commands<'w, 's>,
    pub animations: &'a Res<'a, AnimationAssets>,
    pub scene_handles: &'a Res<'a, SceneAssets>,
}

impl<'a, 'c, 'w, 's> GameObjectSpawner
where
    'c: 'a,
{
    pub fn attach(
        &'c self,
        commands: &'a mut Commands<'w, 's>,
        scenes: &'a Res<'a, Assets<Scene>>,
        materials: &'a Res<'a, Materials>,
        animations: &'a Res<'a, AnimationAssets>,
        scene_handles: &'a Res<'a, SceneAssets>,
    ) -> PrimedGameObjectSpawner<'w, 's, 'a> {
        PrimedGameObjectSpawner {
            outer_spawner: self,
            scenes,
            materials,
            commands,
            animations,
            scene_handles,
        }
    }
}
