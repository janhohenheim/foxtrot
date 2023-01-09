use crate::game_objects::{GameObjectsRetriever, Object};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub const GRASS_SIZE: f32 = 10.;
pub const PATH: &str = "materials/grass.png";

pub fn create_mesh(assets: &mut ResMut<Assets<Mesh>>) -> Handle<Mesh> {
    assets.add(Mesh::from(shape::Box::new(GRASS_SIZE, 0., GRASS_SIZE)))
}

pub fn create_material(
    asset_server: &Res<AssetServer>,
    assets: &mut ResMut<Assets<StandardMaterial>>,
) -> Handle<StandardMaterial> {
    let image: Handle<Image> = asset_server.load(PATH);
    assets.add(image.into())
}

impl<'a> GameObjectsRetriever<'a> {
    pub fn grass(&self, transform: Transform) -> impl Bundle {
        (
            Collider::cuboid(GRASS_SIZE / 2., 0., GRASS_SIZE / 2.),
            PbrBundle {
                mesh: self.game_objects.meshes[&Object::Grass].clone(),
                material: self.game_objects.materials[&Object::Grass].clone(),
                transform,
                ..default()
            },
            Name::new("Grass"),
        )
    }
}
