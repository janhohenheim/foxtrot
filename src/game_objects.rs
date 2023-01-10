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
pub enum GameObject {
    Grass,
}

#[derive(Resource)]
pub struct GameObjectSpawner {
    meshes: HashMap<GameObject, Handle<Mesh>>,
    materials: HashMap<GameObject, Handle<StandardMaterial>>,
}

#[derive(Resource)]
pub struct PrimedGameObjectSpawner<'w, 's, 'a> {
    handles: &'a GameObjectSpawner,
    assets: &'a Res<'a, AssetServer>,
    commands: &'a mut Commands<'w, 's>,
}

impl<'a, 'b> GameObjectSpawner
where
    'b: 'a,
{
    pub fn attach<'w, 's>(
        &'b self,
        asset_server: &'a Res<'a, AssetServer>,
        commands: &'a mut Commands<'w, 's>,
    ) -> PrimedGameObjectSpawner<'w, 's, 'a> {
        PrimedGameObjectSpawner {
            handles: self,
            assets: asset_server,
            commands,
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
    meshes.insert(GameObject::Grass, grass::create_mesh(&mut mesh_assets));

    let mut materials = HashMap::new();
    materials.insert(
        GameObject::Grass,
        grass::create_material(&asset_server, &mut material_assets),
    );

    commands.insert_resource(GameObjectSpawner { meshes, materials });
}
