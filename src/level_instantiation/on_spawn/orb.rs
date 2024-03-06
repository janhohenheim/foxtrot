use crate::{
    level_instantiation::on_spawn::util::MeshAssetsExt, shader::ShaderMaterials, GameState,
};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Component, Reflect, Serialize, Deserialize, Default)]
#[reflect(Component, Serialize, Deserialize)]
struct Orb;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Orb>()
        .add_systems(Update, spawn.run_if(in_state(GameState::Playing)));
}

fn spawn(
    orb: Query<Entity, Added<Orb>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<ShaderMaterials>,
    children: Query<&Children>,
) {
    for entity in orb.iter() {
        let mesh_handle = get_or_add_mesh_handle(&mut meshes);
        children.iter_descendants(entity).for_each(|child| {
            commands.entity(child).despawn_recursive();
        });
        commands
            .entity(entity)
            .insert((MaterialMeshBundle {
                mesh: mesh_handle,
                material: materials.glowy.clone(),
                ..default()
            },))
            .with_children(|parent| {
                parent.spawn((PointLightBundle {
                    point_light: PointLight {
                        intensity: 10_000.,
                        radius: 1.,
                        color: Color::rgb(0.5, 0.1, 0.),
                        shadows_enabled: true,
                        ..default()
                    },
                    ..default()
                },));
            });
    }
}

fn get_or_add_mesh_handle(mesh_assets: &mut Assets<Mesh>) -> Handle<Mesh> {
    const MESH_HANDLE: Handle<Mesh> = Handle::weak_from_u128(0x1f40128bac02a9b);
    mesh_assets.get_or_add(MESH_HANDLE, || Mesh::from(Sphere::new(1.0)))
}
