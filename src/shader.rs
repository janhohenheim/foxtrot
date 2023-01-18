use crate::GameState;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

pub struct ShaderPlugin;

impl Plugin for ShaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MaterialPlugin::<GlowyMaterial>::default())
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_shader));
    }
}

fn spawn_shader(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut glow_materials: ResMut<Assets<GlowyMaterial>>,
) {
    let material = glow_materials.add(GlowyMaterial {});
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::UVSphere {
            radius: 1.0,
            ..default()
        })),
        material: material.clone(),
        ..default()
    });
}

#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "bd5c76fd-6fdd-4de4-9744-4e8beea8daaf"]
pub struct GlowyMaterial {}

impl Material for GlowyMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/glowy.wgsl".into()
    }
}
