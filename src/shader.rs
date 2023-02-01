use crate::GameState;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_resource::{AsBindGroup, ShaderRef, ShaderType};
use bevy::utils::HashMap;
use std::path::Path;

pub struct ShaderPlugin;

impl Plugin for ShaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MaterialPlugin::<GlowyMaterial>::default())
            .add_plugin(MaterialPlugin::<RepeatedMaterial>::default())
            .add_system_set(SystemSet::on_enter(GameState::Loading).with_system(setup_shader));
    }
}

#[derive(Resource, Debug, Clone)]
pub struct Materials {
    pub glowy: Handle<GlowyMaterial>,
    pub repeated: HashMap<Handle<Image>, Handle<RepeatedMaterial>>,
}

fn setup_shader(
    mut commands: Commands,
    mut glow_materials: ResMut<Assets<GlowyMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let env_texture_path = Path::new("hdri").join("stone_alley_2.hdr");
    let env_texture = asset_server.load(env_texture_path);
    let glowy_material = glow_materials.add(GlowyMaterial {
        env_texture: Some(env_texture),
    });

    commands.insert_resource(Materials {
        glowy: glowy_material,
        repeated: HashMap::new(),
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

#[derive(Clone, Copy, ShaderType, Debug)]
pub struct Repeats {
    pub horizontal: f32,
    pub vertical: f32,
}

#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "82d336c5-fd6c-41a3-bdd4-267cd4c9be22"]
pub struct RepeatedMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub texture: Option<Handle<Image>>,
    #[uniform(2)]
    pub repeats: Repeats,
}

impl Material for RepeatedMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/repeated.wgsl".into()
    }
}
