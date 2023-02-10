use crate::file_system_interaction::asset_loading::TextureAssets;
use crate::GameState;
use bevy::asset::{Asset, HandleId};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::render_resource::{AsBindGroup, ShaderRef, ShaderType, Texture};
use bevy::utils::HashMap;
use std::ops::Deref;

pub struct ShaderPlugin;

impl Plugin for ShaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MaterialPlugin::<GlowyMaterial>::default())
            .add_plugin(MaterialPlugin::<RepeatedMaterial>::default())
            // Todo: This somehow calls thread::spawn internally, which breaks WASM
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

    : Res<TextureAssets>,
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
    // Docs for the attributes at <https://github.com/bevyengine/bevy/blob/ee4e98f8a98e1f528065ddaa4a87394715a4c339/crates/bevy_render/src/render_resource/bind_group.rs#L105>
    // The docs disappeared in newer versions. At least I can't find them.
    // Also, this is for some reason only needed on WASM. Weird, since the input is 32 bit precision image and thus does indeed not support filtering.
    #[texture(0, filterable = false)]
    #[sampler(1, sampler_type = "non_filtering")]
    pub env_texture: Option<Handle<Image>>,
}

impl Material for GlowyMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/glowy.wgsl".into()
    }
}

#[derive(Clone, Copy, ShaderType, Debug, Hash, Eq, PartialEq)]
pub struct Repeats {
    pub horizontal: u32,
    pub vertical: u32,
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
