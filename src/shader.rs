use crate::GameState;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use std::path::Path;

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
    asset_server: Res<AssetServer>,
) {
    let env_texture_path = Path::new("hdri").join("stone_alley_2.hdr");
    let env_texture = asset_server.load(env_texture_path);
    let material = glow_materials.add(GlowyMaterial {
        env_texture: Some(env_texture),
    });
    commands
        .spawn(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 1.0,
                ..default()
            })),
            material: material.clone(),
            transform: Transform::from_translation((0., 1.5, 0.).into()),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((PointLightBundle {
                point_light: PointLight {
                    intensity: 10_000.,
                    radius: 1.,
                    color: Color::rgb(0.5, 0.1, 0.),
                    ..default()
                },
                ..default()
            },));
        });
}

#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "bd5c76fd-6fdd-4de4-9744-4e8beea8daaf"]
pub struct GlowyMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub env_texture: Option<Handle<Image>>,
}

impl Material for GlowyMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/glowy.wgsl".into()
    }
}
