use crate::GameState;
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy::utils::HashMap;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::str::FromStr;
use strum_macros::EnumIter;

mod doorway;
pub mod grass;
mod npc;
mod point_light;
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
            .init_resource::<Counter>()
            .register_type::<SpawnEvent>()
            .register_type::<ParentChangeEvent>()
            .register_type::<DuplicationEvent>()
            .register_type::<SpawnTracker>()
            .register_type::<SpawnContainerRegistry>()
            .register_type::<Counter>()
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

impl SpawnEvent {
    pub fn get_name_or_default(&self) -> String {
        self.name
            .clone()
            .map(|name| name.to_string())
            .unwrap_or_else(|| format!("{:?}", self.object))
    }
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
    Debug, EnumIter, Component, Clone, Copy, Eq, PartialEq, Hash, Reflect, Serialize, Deserialize,
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

    commands.insert_resource(GameObjectSpawner {
        meshes,
        materials,
        scenes,
    });
}

#[derive(Debug, Clone, Eq, PartialEq, Resource, Reflect, Serialize, Deserialize, Default)]
#[reflect(Resource, Serialize, Deserialize)]
struct Counter(HashMap<String, usize>);

impl Counter {
    pub fn next(&mut self, name: &str) -> usize {
        *self
            .0
            .entry(name.to_owned())
            .and_modify(|count| *count += 1)
            .or_insert(1)
    }

    pub fn set_at_least(&mut self, name: &str, count: usize) {
        self.0
            .entry(name.to_owned())
            .and_modify(|current| *current = (*current).max(count))
            .or_insert(count);
    }
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
                    SpawnTracker::default(),
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
    name_query: Query<(Entity, &Name), With<SpawnTracker>>,
    spawn_events: EventReader<SpawnEvent>,
    parenting_events: EventReader<ParentChangeEvent>,
    duplication_events: EventReader<DuplicationEvent>,
    mut spawn_containers: ResMut<SpawnContainerRegistry>,
) {
    if spawn_events.is_empty() && parenting_events.is_empty() && duplication_events.is_empty() {
        return;
    }
    spawn_containers.0 = default();

    for (entity, name) in name_query.iter() {
        let name = name.to_string();
        spawn_containers.0.insert(name.clone().into(), entity);
    }
}

fn spawn_requested(
    mut commands: Commands,
    gltf: Res<Assets<Gltf>>,
    mut spawn_requests: EventReader<SpawnEvent>,
    spawner: Res<GameObjectSpawner>,
    mut spawn_containers: ResMut<SpawnContainerRegistry>,
    mut counter: ResMut<Counter>,
) {
    for spawn in spawn_requests.iter() {
        let name = spawn.get_name_or_default();

        let re = Regex::new(r"(^.*) (\d+)$").unwrap();
        if let Some(captures) = re.captures(&name)
            && let Some(name) = captures.get(1).map(|match_| match_.as_str().to_owned())
            && let Some(number) = captures.get(2)
            && let Ok(number) = usize::from_str(number.as_str()) {
            counter.set_at_least(&name, number);
        }

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
            let parent_entity = spawn_containers.get_or_spawn(parent_name.clone(), &mut commands);
            if let Some(&existing_entity) = spawn_containers.0.get::<Cow<'static, str>>(&name.clone().into())
                && matches!(spawn.object, GameObject::Empty) {
                commands.get_entity(existing_entity).unwrap_or_else(|| panic!("Failed to fetch entity with name {}", name)).set_parent(parent_entity).insert(bundle);
            }  else {
                commands.get_entity(parent_entity).unwrap_or_else(|| panic!("Failed to fetch entity with name {}", parent_name)).with_children(|parent| {
                    let entity = parent.spawn(bundle).with_children(spawn_children).id();
                    spawn_containers.0.insert(name.into(), entity);
                });
            }
        } else {
            let entity = commands.spawn(bundle).with_children(spawn_children).id();
            spawn_containers.0.insert(name.into(), entity);
        }
    }
}

fn duplicate(
    mut duplication_requests: EventReader<DuplicationEvent>,
    spawn_containers: Res<SpawnContainerRegistry>,
    mut spawn_requests: EventWriter<SpawnEvent>,
    spawn_tracker_query: Query<(&SpawnTracker, &Name, Option<&Transform>, &Children)>,
    parent_query: Query<&Parent>,
    mut counter: ResMut<Counter>,
) {
    for duplication in duplication_requests.iter() {
        let entity = match spawn_containers.0.get(&duplication.name) {
            None => {
                error!(
                    "Failed to find entity \"{}\" for duplication",
                    duplication.name
                );
                continue;
            }
            Some(entity) => *entity,
        };
        let parent = parent_query
            .get(entity)
            .ok()
            .map(|parent| spawn_tracker_query.get(parent.get()).ok())
            .flatten()
            .map(|(_, name, _, _)| name.to_string().into());
        send_recursive_spawn_events(
            entity,
            parent,
            &mut spawn_requests,
            &spawn_tracker_query,
            &mut counter,
        );
    }
}

fn send_recursive_spawn_events(
    entity: Entity,
    parent: Option<Cow<'static, str>>,
    spawn_requests: &mut EventWriter<SpawnEvent>,
    spawn_tracker_query: &Query<(&SpawnTracker, &Name, Option<&Transform>, &Children)>,
    counter: &mut ResMut<Counter>,
) {
    let (spawn_tracker, name, transform, children) = match spawn_tracker_query.get(entity) {
        Ok(result) => result,
        Err(_) => {
            return;
        }
    };

    let name = name.to_string();

    let re = Regex::new(r"^(.*) \d+$").unwrap();
    let name = if let Some(captures) = re.captures(&name) && let Some(unnumbered_name) = captures.get(1) {
        unnumbered_name.as_str()
    } else {
        &name
    };
    let number = counter.next(name);
    let name = Some(format!("{} {}", name, number).into());

    spawn_requests.send(SpawnEvent {
        object: spawn_tracker.object,
        transform: transform.map(Clone::clone).unwrap_or_default(),
        parent,
        name: name.clone(),
    });
    for &child in children {
        send_recursive_spawn_events(
            child,
            name.clone(),
            spawn_requests,
            spawn_tracker_query,
            counter,
        );
    }
}

fn change_parent(
    mut commands: Commands,
    mut parent_changes: EventReader<ParentChangeEvent>,
    mut spawn_containers: ResMut<SpawnContainerRegistry>,
) {
    for change in parent_changes.iter() {
        let child = match spawn_containers.0.get(&change.name) {
            None => {
                error!("Failed to fetch child: {}", change.name);
                continue;
            }
            Some(&entity) => entity,
        };

        if let Some(parent) = change.new_parent.clone() {
            let parent = spawn_containers.get_or_spawn(parent, &mut commands);
            commands
                .get_entity(child)
                .unwrap_or_else(|| panic!("Failed to fetch entity with name {}", change.name))
                .set_parent(parent);
        } else {
            commands
                .get_entity(child)
                .unwrap_or_else(|| panic!("Failed to fetch entity with name {}", change.name))
                .remove_parent();
        }
    }
}
