use crate::GameState;
use bevy::ecs::system::EntityCommands;
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use strum_macros::EnumIter;

mod doorway;
pub mod grass;
mod npc;
mod primitives;
mod roof;
mod roof_left;
mod roof_right;
mod sunlight;
mod util;
mod wall;

pub struct SpawningPlugin;

impl Plugin for SpawningPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnEvent>()
            .add_event::<ParentChangeEvent>()
            .add_event::<DuplicationEvent>()
            .init_resource::<SpawnContainerRegistry>()
            .register_type::<SpawnEvent>()
            .register_type::<ParentChangeEvent>()
            .register_type::<DuplicationEvent>()
            .register_type::<SpawnTracker>()
            .register_type::<SpawnContainerRegistry>()
            .add_startup_system(load_assets_for_spawner)
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(spawn_requested.label("spawn_requested"))
                    .with_system(sync_container_registry.before("spawn_requested"))
                    .with_system(change_parent.after("spawn_requested"))
                    .with_system(duplicate.after("spawn_requested")),
            );
    }
}

#[derive(Debug, Clone, PartialEq, Default, Reflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub struct ParentChangeEvent {
    pub name: Cow<'static, str>,
    pub new_parent: Option<Cow<'static, str>>,
}

#[derive(Debug, Clone, PartialEq, Default, Reflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub struct DuplicationEvent {
    pub name: Cow<'static, str>,
}

#[derive(Debug, Component, Clone, PartialEq, Default, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct SpawnEvent {
    pub object: GameObject,
    pub transform: Transform,
    #[serde(default)]
    pub parent: Option<Cow<'static, str>>,
    #[serde(default)]
    pub name: Option<Cow<'static, str>>,
}

#[derive(Debug, Component, Clone, PartialEq, Default, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct SpawnTracker {
    pub object: GameObject,
    pub name: Option<Cow<'static, str>>,
}

impl From<SpawnEvent> for SpawnTracker {
    fn from(value: SpawnEvent) -> Self {
        Self {
            object: value.object,
            name: value.name,
        }
    }
}

#[derive(
    Debug, EnumIter, Component, Clone, Copy, Eq, PartialEq, Hash, Reflect, Serialize, Deserialize,
)]
#[reflect(Component, Serialize, Deserialize)]
pub enum GameObject {
    Empty,
    Box,
    Sphere,
    Capsule,
    Grass,
    Doorway,
    Wall,
    Roof,
    RoofRight,
    RoofLeft,
    Sunlight,
    Npc,
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

impl<'w, 's, 'a, 'b> PrimedGameObjectSpawner<'w, 's, 'a, 'b> {
    pub fn spawn(&'a mut self, object: &GameObject) -> EntityCommands<'w, 's, 'a> {
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
        }
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

    commands.insert_resource(GameObjectSpawner {
        meshes,
        materials,
        scenes,
    });
}

#[derive(Debug, Clone, Eq, PartialEq, Resource, Reflect, Serialize, Deserialize, Default)]
#[reflect(Resource, Serialize, Deserialize)]
struct SpawnContainerRegistry(HashMap<Cow<'static, str>, Entity>);

impl SpawnContainerRegistry {
    pub fn get_or_spawn(
        &mut self,
        name: impl Into<Cow<'static, str>>,
        commands: &mut Commands,
    ) -> Entity {
        // command.spawn() takes a tick to actually spawn stuff,
        // so we need to keep an own list of already "spawned" parents
        let name = name.into();
        let spawn_parent = |commands: &mut Commands| {
            commands
                .spawn((
                    Name::new(name.clone()),
                    VisibilityBundle::default(),
                    TransformBundle::default(),
                    SpawnTracker {
                        name: Some(name.clone()),
                        ..default()
                    },
                ))
                .id()
        };
        let parent = self
            .0
            .entry(name.clone())
            .or_insert_with(|| spawn_parent(commands))
            .clone();

        if commands.get_entity(parent).is_some() {
            parent
        } else {
            // parent was removed at some prior point
            let entity = spawn_parent(commands);
            self.0.insert(name.clone(), entity);
            entity
        }
    }
}

fn sync_container_registry(
    name_query: Query<(Entity, &Name), Changed<Name>>,
    removed_names: RemovedComponents<Name>,
    mut spawn_containers: ResMut<SpawnContainerRegistry>,
) {
    for (entity, name) in name_query.iter() {
        let name = name.to_string();
        spawn_containers.0.insert(name.into(), entity);
    }
    for removed_entity in removed_names.iter() {
        let names: Vec<_> = spawn_containers
            .0
            .iter()
            .filter_map(|(name, entity)| (*entity == removed_entity).then(|| name.clone()))
            .collect();
        for name in names {
            spawn_containers.0.remove(&name);
        }
    }
}

fn spawn_requested(
    mut commands: Commands,
    gltf: Res<Assets<Gltf>>,
    mut spawn_requests: EventReader<SpawnEvent>,
    spawner: Res<GameObjectSpawner>,
    mut spawn_containers: ResMut<SpawnContainerRegistry>,
) {
    for spawn in spawn_requests.iter() {
        let name = spawn
            .name
            .clone()
            .map(|name| name.to_string())
            .unwrap_or_else(|| format!("{:?}", spawn.object));

        let bundle = (
            Name::new(name.clone()),
            VisibilityBundle::default(),
            TransformBundle::from_transform(spawn.transform),
            SpawnTracker::from(spawn.clone()),
        );
        let spawn_children = |parent: &mut ChildBuilder| {
            spawner.attach(parent, &gltf).spawn(&spawn.object);
        };

        if let Some(ref parent_name) = spawn.parent {
            let entity = spawn_containers.get_or_spawn(parent_name.clone(), &mut commands);
            commands.entity(entity).with_children(|parent| {
                parent.spawn(bundle).with_children(spawn_children);
            });
        } else {
            let identity = commands.spawn(bundle).with_children(spawn_children).id();
            spawn_containers.0.insert(name.into(), identity);
        }
    }
}

fn duplicate(
    mut duplication_requests: EventReader<DuplicationEvent>,
    spawn_containers: Res<SpawnContainerRegistry>,
    mut spawn_requests: EventWriter<SpawnEvent>,
    spawn_tracker_query: Query<(&SpawnTracker, Option<&Transform>, &Children)>,
) {
    for duplication in duplication_requests.iter() {
        let entity = spawn_containers.0.get(&duplication.name).unwrap();

        send_recursive_spawn_events(*entity, None, &mut spawn_requests, &spawn_tracker_query);
    }
}

fn send_recursive_spawn_events(
    entity: Entity,
    parent: Option<Cow<'static, str>>,
    spawn_requests: &mut EventWriter<SpawnEvent>,
    spawn_tracker_query: &Query<(&SpawnTracker, Option<&Transform>, &Children)>,
) {
    let (spawn_tracker, transform, children) = match spawn_tracker_query.get(entity) {
        Ok(result) => result,
        Err(_) => {
            return;
        }
    };
    spawn_requests.send(SpawnEvent {
        object: spawn_tracker.object,
        transform: transform.map(Clone::clone).unwrap_or_default(),
        parent,
        name: spawn_tracker.name.clone(),
    });
    for &child in children {
        send_recursive_spawn_events(
            child,
            spawn_tracker.name.clone(),
            spawn_requests,
            spawn_tracker_query,
        );
    }
}

fn change_parent(
    mut commands: Commands,
    mut parent_changes: EventReader<ParentChangeEvent>,
    mut spawn_containers: ResMut<SpawnContainerRegistry>,
) {
    for change in parent_changes.iter() {
        let child = *spawn_containers.0.get(&change.name).unwrap();
        if let Some(parent) = change.new_parent.clone() {
            let parent = spawn_containers.get_or_spawn(parent, &mut commands);
            commands.entity(child).set_parent(parent);
        } else {
            commands.entity(child).remove_parent();
        }
    }
}
