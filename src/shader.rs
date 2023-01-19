use crate::GameState;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use std::path::Path;

pub struct ShaderPlugin;

impl Plugin for ShaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MaterialPlugin::<GlowyMaterial>::default())
            .add_plugin(MaterialPlugin::<RepeatedMaterial>::default())
            .add_system_set(SystemSet::on_enter(GameState::Loading).with_system(setup_shader))
            .add_system_set(SystemSet::on_enter(GameState::Playing).with_system(spawn_shader))
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(apply_shader)
                    .with_system(set_texture_to_repeat),
            );
    }
}

fn setup_shader(
    mut commands: Commands,
    mut glow_materials: ResMut<Assets<GlowyMaterial>>,
    mut repeated_materials: ResMut<Assets<RepeatedMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let env_texture_path = Path::new("hdri").join("stone_alley_2.hdr");
    let env_texture = asset_server.load(env_texture_path);
    let glowy_material = glow_materials.add(GlowyMaterial {
        env_texture: Some(env_texture),
    });

    let texture_path = Path::new("textures").join("ground_forest.png");
    let texture = asset_server.load(texture_path);
    let repeated_material = repeated_materials.add(RepeatedMaterial {
        texture: Some(texture),
    });
    commands.insert_resource(Materials {
        glowy: glowy_material,
        repeated: repeated_material,
    });
}

#[derive(Resource, Debug, Clone)]
struct Materials {
    pub glowy: Handle<GlowyMaterial>,
    pub repeated: Handle<RepeatedMaterial>,
}

fn spawn_shader(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<Materials>,
) {
    commands
        .spawn(MaterialMeshBundle {
            mesh: meshes.add(Mesh::from(shape::UVSphere {
                radius: 1.0,
                ..default()
            })),
            material: materials.glowy.clone(),
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

#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "82d336c5-fd6c-41a3-bdd4-267cd4c9be22"]
pub struct RepeatedMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub texture: Option<Handle<Image>>,
}

impl Material for RepeatedMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/repeated.wgsl".into()
    }
}

#[allow(clippy::type_complexity)]
fn apply_shader(
    mut _commands: Commands,
    _added_name: Query<(Entity, &Name), Added<Name>>,
    _materials: Res<Materials>,
) {
    /*for (entity, name) in &added_name {
        if name.to_lowercase().contains("player") {
            commands.entity(entity).insert(materials.glowy.clone());
        }
    }*/
}

fn set_texture_to_repeat(
    mut commands: Commands,
    added_name: Query<(&Name, &Children), Added<Name>>,
    material_handles: Query<&Handle<StandardMaterial>>,
    materials: Res<Materials>,
) {
    for (name, children) in &added_name {
        if name.to_lowercase().contains("ground") {
            let child = children
                .iter()
                .find(|entity| material_handles.get(**entity).is_ok())
                .unwrap();

            commands
                .entity(*child)
                .remove::<Handle<StandardMaterial>>()
                .insert(materials.repeated.clone());
        }
    }
}
