use crate::GameState;
use bevy::ecs::system::EntityCommands;
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use strum_macros::EnumIter;

mod doorway;
mod empty;
pub mod grass;
mod npc;
mod sunlight;
mod wall;

pub struct SpawningPlugin;

impl Plugin for SpawningPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnEvent>()
            .init_resource::<SpawnContainerRegistry>()
            .add_startup_system(load_assets_for_spawner)
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(spawn_requested));
    }
}

#[derive(Debug, Component, Clone, PartialEq, Default, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct SpawnEvent {
    pub object: GameObject,
    pub transform: Transform,
    pub parent: Option<Cow<'static, str>>,
}

#[derive(Debug, Component, Clone, PartialEq, Default, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct SpawnTracker {
    pub object: GameObject,
    pub parent: Option<Cow<'static, str>>,
}
impl From<SpawnEvent> for SpawnTracker {
    fn from(value: SpawnEvent) -> Self {
        Self {
            object: value.object,
            parent: value.parent,
        }
    }
}

pub struct SpawnEventSender<'a, 'w, 's> {
    pub object: Option<GameObject>,
    pub transform: Transform,
    pub parent: Option<Cow<'static, str>>,
    pub event_writer: &'a mut EventWriter<'w, 's, SpawnEvent>,
}

impl<'a, 'w, 's> SpawnEventSender<'a, 'w, 's> {
    pub fn new(event_writer: &'a mut EventWriter<'w, 's, SpawnEvent>) -> Self {
        Self {
            event_writer,
            object: default(),
            transform: default(),
            parent: default(),
        }
    }

    pub fn transform(&mut self, transform: Transform) -> &mut Self {
        self.transform = transform;
        self
    }

    pub fn translation(&mut self, translation: impl Into<Vec3>) -> &mut Self {
        self.transform.translation = translation.into();
        self
    }

    pub fn rotation(&mut self, rotation: impl Into<Quat>) -> &mut Self {
        self.transform.rotation = rotation.into();
        self
    }

    pub fn object(&mut self, object: GameObject) -> &mut Self {
        self.object = Some(object);
        self
    }
    pub fn parent(&mut self, parent: impl Into<Cow<'static, str>>) -> &mut Self {
        self.parent = Some(parent.into());
        self
    }

    pub fn send(&mut self) -> &mut Self {
        let event = SpawnEvent {
            object: self.object.expect(
                "Called SpawnEventSender::send() without calling SpawnEventSender::object() first",
            ),
            transform: self.transform,
            parent: self.parent.clone(),
        };
        self.event_writer.send(event);
        self
    }
}

#[derive(
    Debug, EnumIter, Component, Clone, Copy, Eq, PartialEq, Hash, Reflect, Serialize, Deserialize,
)]
#[reflect(Component, Serialize, Deserialize)]
pub enum GameObject {
    Grass,
    Doorway,
    Wall,
    Sunlight,
    Npc,
    Empty,
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
            GameObject::Sunlight => self.spawn_sunlight(),
            GameObject::Npc => self.spawn_npc(),
            GameObject::Empty => self.spawn_empty(),
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

fn spawn_requested(
    mut commands: Commands,
    gltf: Res<Assets<Gltf>>,
    mut spawn_requests: EventReader<SpawnEvent>,
    spawner: Res<GameObjectSpawner>,
    mut spawn_containers: ResMut<SpawnContainerRegistry>,
) {
    for spawn in spawn_requests.iter() {
        let bundle = (
            Name::new(format!("{:?}", spawn.object)),
            VisibilityBundle::default(),
            TransformBundle::from_transform(spawn.transform),
            SpawnTracker::from(spawn.clone()),
        );
        let spawn_children = |parent: &mut ChildBuilder| {
            spawner.attach(parent, &gltf).spawn(&spawn.object);
        };

        if let Some(ref parent_name) = spawn.parent {
            // command.spawn() takes a tick to actually spawn stuff,
            // so we need to keep an own list of already "spawned" parents
            let parent = spawn_containers
                .0
                .entry(parent_name.to_owned())
                .or_insert_with(|| {
                    commands
                        .spawn((
                            Name::new(parent_name.clone()),
                            VisibilityBundle::default(),
                            TransformBundle::default(),
                        ))
                        .id()
                });

            commands.entity(*parent).with_children(|parent| {
                parent.spawn(bundle).with_children(spawn_children);
            });
        } else {
            commands.spawn(bundle).with_children(spawn_children);
        }
    }
}
