use bevy::prelude::*;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};
mod grass;
use strum_macros::EnumIter;

pub struct GameObjectsPlugin;

impl Plugin for GameObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_game_objects);
    }
}

#[derive(Debug, EnumIter, Clone, Copy, Eq, PartialEq, Hash, Reflect, Serialize, Deserialize)]
#[reflect(Serialize, Deserialize)]
pub enum Object {
    Grass,
}

#[derive(Resource)]
pub struct GameObjects {
    meshes: HashMap<Object, Handle<Mesh>>,
    materials: HashMap<Object, Handle<StandardMaterial>>,
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

impl<'a> GameObjectsRetriever<'a> {
    pub fn get(&self, object: &Object, transform: Transform) -> impl Bundle {
        match *object {
            Object::Grass => self.grass(transform),
        }
    }
}

fn setup_game_objects(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut mesh_assets: ResMut<Assets<Mesh>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
) {
    let mut meshes = HashMap::new();
    meshes.insert(Object::Grass, grass::create_mesh(&mut mesh_assets));

    let mut materials = HashMap::new();
    materials.insert(
        Object::Grass,
        grass::create_material(&asset_server, &mut material_assets),
    );

    commands.insert_resource(GameObjects { meshes, materials });
}
