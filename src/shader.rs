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
            .add_system_set(SystemSet::on_enter(GameState::Loading).with_system(setup_shader));
    }
}

#[derive(Resource, Debug, Clone)]
pub struct Materials {
    pub glowy: Handle<GlowyMaterial>,
    pub repeated: Handle<RepeatedMaterial>,
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
