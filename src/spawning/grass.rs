use crate::spawning::{GameObject, PrimedGameObjectSpawner};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub const GRASS_SIZE: f32 = 10.;
pub const PATH: &str = "materials/grass.png";

pub fn create_mesh(assets: &mut ResMut<Assets<Mesh>>) -> Handle<Mesh> {
    assets.add(Mesh::from(shape::Plane { size: GRASS_SIZE }))
}

pub fn load_material(
    asset_server: &Res<AssetServer>,
    assets: &mut ResMut<Assets<StandardMaterial>>,
) -> Handle<StandardMaterial> {
    let image: Handle<Image> = asset_server.load(PATH);
    assets.add(image.into())
}

impl<'w, 's, 'a, 'b> PrimedGameObjectSpawner<'w, 's, 'a, 'b> {
    pub fn spawn_grass(&'a mut self) {
        const HALF_EXTENT: Vec3 = Vec3::new(GRASS_SIZE / 2., 0., GRASS_SIZE / 2.);
        self.commands.spawn((
            Collider::cuboid(HALF_EXTENT.x, HALF_EXTENT.y, HALF_EXTENT.z),
            PbrBundle {
                mesh: self.handles.meshes[&GameObject::Grass].clone(),
                material: self.handles.materials[&GameObject::Grass].clone(),
                transform: Transform::from_translation(HALF_EXTENT),
                ..default()
            },
        ));
    }
}
