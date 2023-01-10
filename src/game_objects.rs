use bevy::ecs::system::EntityCommands;
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
pub struct GameObjectsRetriever<'w, 's, 'a> {
    game_objects: &'a GameObjects,
    asset_server: &'a Res<'a, AssetServer>,
    commands: &'a mut Commands<'w, 's>,
}

impl<'a, 'b> GameObjects
where
    'b: 'a,
{
    pub fn retrieve_with<'w, 's>(
        &'b self,
        asset_server: &'a Res<'a, AssetServer>,
        commands: &'a mut Commands<'w, 's>,
    ) -> GameObjectsRetriever<'w, 's, 'a> {
        GameObjectsRetriever {
            game_objects: self,
            asset_server,
            commands,
        }
    }
}

impl<'w, 's, 'a> GameObjectsRetriever<'w, 's, 'a> {
    pub fn spawn(
        &'a mut self,
        object: &Object,
        transform: Transform,
    ) -> EntityCommands<'w, 's, 'a> {
        match *object {
            Object::Grass => self.spawn_grass(transform),
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
