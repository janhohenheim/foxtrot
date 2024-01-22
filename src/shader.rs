#![allow(clippy::extra_unused_type_parameters)]
use crate::file_system_interaction::asset_loading::TextureAssets;
use crate::GameState;
use anyhow::Result;

use bevy::pbr::{MaterialPipeline, MaterialPipelineKey};
use bevy::prelude::*;

use bevy::render::mesh::MeshVertexBufferLayout;
use bevy::render::render_resource::Face::Front;
use bevy::render::render_resource::{
    AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
};

/// Handles instantiation of shaders. The shaders can be found in the [`shaders`](https://github.com/janhohenheim/foxtrot/tree/main/assets/shaders) directory.
/// Shaders are stored in [`Material`]s which can be used on objects by attaching a `Handle<Material>` to an entity.
/// The handles can be stored and retrieved in the [`Materials`] resource.
pub(crate) fn shader_plugin(app: &mut App) {
    app.add_plugins(MaterialPlugin::<GlowyMaterial>::default())
        .add_plugins(MaterialPlugin::<SkydomeMaterial>::default())
        .add_systems(OnExit(GameState::Loading), setup_shader);
}

#[derive(Resource, Debug, Clone)]
pub(crate) struct Materials {
    pub(crate) glowy: Handle<GlowyMaterial>,
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

    commands.insert_resource(Materials { glowy, skydome });
}

#[derive(AsBindGroup, Debug, Clone, Asset, TypePath)]
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

#[derive(AsBindGroup, Debug, Clone, Asset, TypePath)]
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
