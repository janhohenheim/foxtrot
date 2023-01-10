use crate::spawning::{GameObject, PrimedGameObjectSpawner};
use bevy::ecs::system::EntityCommands;
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

impl<'w, 's, 'a> PrimedGameObjectSpawner<'w, 's, 'a> {
    pub fn spawn_grass(&'a mut self, transform: Transform) -> EntityCommands<'w, 's, 'a> {
        let mut entity_commands = self.commands.spawn((
            Name::new("Grass"),
            VisibilityBundle::default(),
            TransformBundle::from_transform(transform),
        ));
        entity_commands.with_children(|parent| {
            const HALF_EXTENT: Vec3 = Vec3::new(GRASS_SIZE / 2., 0., GRASS_SIZE / 2.);
            parent.spawn((
                Collider::cuboid(HALF_EXTENT.x, HALF_EXTENT.y, HALF_EXTENT.z),
                PbrBundle {
                    mesh: self.handles.meshes[&GameObject::Grass].clone(),
                    material: self.handles.materials[&GameObject::Grass].clone(),
                    transform: Transform::from_translation(HALF_EXTENT),
                    ..default()
                },
            ));
        });
        entity_commands
    }
}
