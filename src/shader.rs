use crate::{file_system_interaction::asset_loading::TextureAssets, GameState};
use anyhow::Result;

use bevy::prelude::*;

use bevy::render::render_resource::{AsBindGroup, ShaderRef};

/// Handles instantiation of shaders. The shaders can be found in the [`shaders`](https://github.com/janhohenheim/foxtrot/tree/main/assets/shaders) directory.
/// Shaders are stored in [`Material`]s which can be used on objects by attaching a `Handle<Material>` to an entity.
/// The handles can be stored and retrieved in the [`ShaderMaterials`] resource.
pub(super) fn plugin(app: &mut App) {
    app.add_plugins(MaterialPlugin::<GlowyMaterial>::default())
        .add_systems(OnExit(GameState::Loading), setup_shader);
}

#[derive(Resource, Debug, Clone)]
pub(crate) struct ShaderMaterials {
    pub(crate) glowy: Handle<GlowyMaterial>,
}

fn setup_shader(
    mut commands: Commands,
    mut glow_materials: ResMut<Assets<GlowyMaterial>>,
    texture_assets: Res<TextureAssets>,
) {
    let glowy = glow_materials.add(GlowyMaterial {
        env_texture: texture_assets.glowy_interior.clone(),
    });

    commands.insert_resource(ShaderMaterials { glowy });
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
