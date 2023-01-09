use crate::game_objects::{GameObjectsRetriever, Objects};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub const GRASS_SIZE: f32 = 10.;
pub const PATH: &str = "materials/grass.png";

pub fn create_mesh(meshes: &mut ResMut<Assets<Mesh>>) -> Handle<Mesh> {
    meshes.add(Mesh::from(shape::Box::new(GRASS_SIZE, 0., GRASS_SIZE)))
}

impl<'a> GameObjectsRetriever<'a> {
    pub fn grass(&self, transform: Transform) -> impl Bundle {
        let material = self.asset_server.load(PATH);
        let Vec3 { x, y, z } = transform.translation;
        (
            Collider::cuboid(x / 2., y / 2., z / 2.),
            PbrBundle {
                mesh: self.game_objects.meshes[&Objects::Grass].clone(),
                material,
                transform,
                ..default()
            },
            Name::new("Grass"),
        )
    }
}
