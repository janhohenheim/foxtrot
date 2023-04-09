#![allow(clippy::extra_unused_type_parameters)]
use crate::file_system_interaction::asset_loading::TextureAssets;
use crate::GameState;
use anyhow::{Context, Result};
use bevy::asset::HandleId;
use bevy::pbr::{MaterialPipeline, MaterialPipelineKey};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::mesh::MeshVertexBufferLayout;
use bevy::render::render_resource::Face::Front;
use bevy::render::render_resource::{
    AsBindGroup, RenderPipelineDescriptor, ShaderRef, ShaderType, SpecializedMeshPipelineError,
};
use bevy::utils::HashMap;
use bevy_mod_sysfail::macros::*;
use regex::Regex;
use std::sync::LazyLock;

/// Handles instantiation of shaders. The shaders can be found in the [`shaders`](https://github.com/janhohenheim/foxtrot/tree/main/assets/shaders) directory.
/// Shaders are stored in [`Material`]s which can be used on objects by attaching a `Handle<Material>` to an entity.
/// The handles can be stored and retrieved in the [`Materials`] resource.
pub(crate) fn shader_plugin(app: &mut App) {
    app.add_plugin(MaterialPlugin::<GlowyMaterial>::default())
        .add_plugin(MaterialPlugin::<RepeatedMaterial>::default())
        .add_plugin(MaterialPlugin::<SkydomeMaterial>::default())
        .add_system(setup_shader.in_schedule(OnExit(GameState::Loading)))
        .add_system(set_texture_to_repeat.in_set(OnUpdate(GameState::Playing)));
}

#[derive(Resource, Debug, Clone)]
pub(crate) struct Materials {
    pub(crate) glowy: Handle<GlowyMaterial>,
    /// (Texture asset ID, Repeats) -> RepeatedMaterial
    pub(crate) repeated: HashMap<(HandleId, Repeats), Handle<RepeatedMaterial>>,
    pub(crate) skydome: Handle<SkydomeMaterial>,
}

fn setup_shader(
    mut commands: Commands,
    mut glow_materials: ResMut<Assets<GlowyMaterial>>,
    mut skydome_materials: ResMut<Assets<SkydomeMaterial>>,
    texture_assets: Res<TextureAssets>,
) {
    let glowy = glow_materials.add(GlowyMaterial {
        env_texture: texture_assets.glowy_interior.clone(),
    });
    let skydome = skydome_materials.add(SkydomeMaterial {
        env_texture: texture_assets.sky.clone(),
    });

    commands.insert_resource(Materials {
        repeated: HashMap::new(),
        glowy,
        skydome,
    });
}

#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "bd5c76fd-6fdd-4de4-9744-4e8beea8daaf"]
/// Material for [`glowy.wgsl`](https://github.com/janhohenheim/foxtrot/blob/main/assets/shaders/glowy.wgsl).
pub(crate) struct GlowyMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub(crate) env_texture: Handle<Image>,
}

impl Material for GlowyMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/glowy.wgsl".into()
    }
}

#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "8ca95d76-91d6-44c0-a67b-8a4d22cd59b1"]
/// Material for [`skydome.wgsl`](https://github.com/janhohenheim/foxtrot/blob/main/assets/shaders/skydome.wgsl).
pub(crate) struct SkydomeMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub(crate) env_texture: Handle<Image>,
}

impl Material for SkydomeMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/skydome.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = Some(Front);
        Ok(())
    }
}

#[repr(C, align(16))] // All WebGPU uniforms must be aligned to 16 bytes
#[derive(Clone, Copy, ShaderType, Debug, Hash, Eq, PartialEq, Default)]
pub(crate) struct Repeats {
    pub(crate) horizontal: u32,
    pub(crate) vertical: u32,
    pub(crate) _wasm_padding1: u32,
    pub(crate) _wasm_padding2: u32,
}

#[derive(AsBindGroup, Debug, Clone, TypeUuid)]
#[uuid = "82d336c5-fd6c-41a3-bdd4-267cd4c9be22"]
/// Material for [`repeated.wgsl`](https://github.com/janhohenheim/foxtrot/blob/main/assets/shaders/repeated.wgsl).
pub(crate) struct RepeatedMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub(crate) texture: Handle<Image>,
    #[uniform(2)]
    pub(crate) repeats: Repeats,
}

impl Material for RepeatedMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/repeated.wgsl".into()
    }
}

static REPEAT_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\[repeat:\s*(\d+),\s*(\d+)\]").expect("Failed to compile repeat regex")
});

#[sysfail(log(level = "error"))]
pub(crate) fn set_texture_to_repeat(
    mut commands: Commands,
    added_name: Query<(&Name, &Children), Added<Name>>,
    material_handles: Query<&Handle<StandardMaterial>>,
    mut materials: ResMut<Materials>,
    standard_materials: Res<Assets<StandardMaterial>>,
    mut repeated_materials: ResMut<Assets<RepeatedMaterial>>,
) -> Result<()> {
    for (name, children) in &added_name {
        if let Some(captures) = REPEAT_REGEX.captures(&name.to_lowercase()) {
            let repeats = Repeats {
                horizontal: captures[1].parse().context("Failed to parse repeat")?,
                vertical: captures[2].parse().context("Failed to parse repeat")?,
                ..default()
            };
            for child in children.iter() {
                if let Ok(standard_material_handle) = material_handles.get(*child) {
                    let standard_material = standard_materials
                        .get(standard_material_handle)
                        .context("Failed to get standard material from handle")?;
                    let texture = standard_material.base_color_texture.as_ref().context(
                        "Failed to get texture from standard material. Is the texture missing?",
                    )?;
                    let key = (texture.id(), repeats);

                    let repeated_material = materials.repeated.entry(key).or_insert_with(|| {
                        repeated_materials.add(RepeatedMaterial {
                            texture: texture.clone(),
                            repeats,
                        })
                    });

                    commands
                        .entity(*child)
                        .remove::<Handle<StandardMaterial>>()
                        .insert(repeated_material.clone());
                }
            }
        }
    }
    Ok(())
}
