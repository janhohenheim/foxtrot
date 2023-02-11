use crate::file_system_interaction::asset_loading::TextureAssets;
use crate::GameState;
use bevy::asset::HandleId;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_resource::{AsBindGroup, ShaderRef, ShaderType};
use bevy::utils::HashMap;

pub struct ShaderPlugin;

impl Plugin for ShaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MaterialPlugin::<GlowyMaterial>::default())
            .add_plugin(MaterialPlugin::<RepeatedMaterial>::default())
            .add_system_set(SystemSet::on_exit(GameState::Loading).with_system(setup_shader));
    }
}

#[derive(Resource, Debug, Clone)]
pub struct Materials {
    pub glowy: Handle<GlowyMaterial>,
    /// (Texture asset ID, Repeats) -> RepeatedMaterial
    pub repeated: HashMap<(HandleId, Repeats), Handle<RepeatedMaterial>>,
}

fn setup_shader(
    mut commands: Commands,
    mut glow_materials: ResMut<Assets<GlowyMaterial>>,
    texture_assets: Res<TextureAssets>,
) {
    let glowy_material = glow_materials.add(GlowyMaterial {
        env_texture: Some(texture_assets.glowy_interior.clone()),
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

#[repr(align(16))] // All WebGPU uniforms must be aligned to 16 bytes
#[derive(Clone, Copy, ShaderType, Debug, Hash, Eq, PartialEq, Default)]
#[allow(clippy::extra_unused_type_parameters)]
pub struct Repeats {
    pub horizontal: u32,
    pub vertical: u32,
    pub _wasm_padding1: u32,
    pub _wasm_padding2: u32,
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
