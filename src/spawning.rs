use bevy::ecs::system::EntityCommands;
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};
mod doorway;
mod grass;
use crate::GameState;
use strum_macros::EnumIter;

pub struct GameObjectsPlugin;

impl Plugin for GameObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnEvent>()
            .add_startup_system(load_assets_for_spawner)
            .add_system_set(SystemSet::on_update(GameState::Playing).with_system(spawn_requested));
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpawnEvent {
    pub object: GameObject,
    pub transform: Transform,
}

#[derive(Debug, EnumIter, Clone, Copy, Eq, PartialEq, Hash, Reflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub enum GameObject {
    Grass,
    Doorway,
}

#[derive(Resource)]
pub struct GameObjectSpawner {
    meshes: HashMap<GameObject, Handle<Mesh>>,
    materials: HashMap<GameObject, Handle<StandardMaterial>>,
    scenes: HashMap<GameObject, Handle<Gltf>>,
}

#[derive(Resource)]
pub struct PrimedGameObjectSpawner<'w, 's, 'a> {
    handles: &'a GameObjectSpawner,
    assets: &'a Res<'a, AssetServer>,
    gltf: &'a Res<'a, Assets<Gltf>>,
    commands: &'a mut Commands<'w, 's>,
}

impl<'a, 'b> GameObjectSpawner
where
    'b: 'a,
{
    pub fn attach<'w, 's>(
        &'b self,
        assets: &'a Res<'a, AssetServer>,
        commands: &'a mut Commands<'w, 's>,
        gltf: &'a Res<'a, Assets<Gltf>>,
    ) -> PrimedGameObjectSpawner<'w, 's, 'a> {
        PrimedGameObjectSpawner {
            handles: self,
            assets,
            commands,
            gltf,
        }
    }
}

impl<'w, 's, 'a> PrimedGameObjectSpawner<'w, 's, 'a> {
    pub fn spawn(
        &'a mut self,
        object: &GameObject,
        transform: Transform,
    ) -> EntityCommands<'w, 's, 'a> {
        match *object {
            GameObject::Grass => self.spawn_grass(transform),
            GameObject::Doorway => self.spawn_doorway(transform),
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

    commands.insert_resource(GameObjectSpawner {
        meshes,
        materials,
        scenes,
    });
}

fn spawn_requested(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    gltf: Res<Assets<Gltf>>,
    mut spawn_requests: EventReader<SpawnEvent>,
    spawner: Res<GameObjectSpawner>,
) {
    for spawn in spawn_requests.iter() {
        spawner
            .attach(&asset_server, &mut commands, &gltf)
            .spawn(&spawn.object, spawn.transform);
    }
}
