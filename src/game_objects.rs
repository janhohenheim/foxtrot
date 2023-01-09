use crate::loading::{MaterialAssets, SceneAssets};
use bevy::gltf::Gltf;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_rapier3d::prelude::*;
use serde::{Deserialize, Serialize};
mod grass;

pub struct GameObjectsPlugin;

impl Plugin for GameObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_game_objects);
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Reflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub enum Objects {
    Grass,
}

#[derive(Resource)]
pub struct GameObjects {
    meshes: HashMap<Objects, Handle<Mesh>>,
}

#[derive(Resource)]
pub struct GameObjectsRetriever<'a> {
    game_objects: &'a GameObjects,
    asset_server: Res<'a, AssetServer>,
}

impl<'a, 'b> GameObjects
where
    'b: 'a,
{
    pub fn retrieve_with(&'b self, asset_server: Res<'a, AssetServer>) -> GameObjectsRetriever<'a> {
        GameObjectsRetriever {
            game_objects: self,
            asset_server,
        }
    }
}

fn setup_game_objects(mut commands: Commands, mut mesh_assets: ResMut<Assets<Mesh>>) {
    let mut meshes = HashMap::new();
    meshes.insert(Objects::Grass, grass::create_mesh(&mut mesh_assets));
    commands.insert_resource(GameObjects { meshes });
}
