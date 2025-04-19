use bevy::{ecs::world::DeferredWorld, prelude::*};
use bevy_trenchbroom::{bsp::base_classes::BspWorldspawn, class::QuakeClass, prelude::*};
use proxy::RegisterProxies as _;

use crate::{
    gameplay::{npc::Npc, player::Player},
    props::RegisterProps as _,
};

mod proxy;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(TrenchBroomPlugin({
        let config = TrenchBroomConfig::new("foxtrot")
            .generic_material_extension("material.toml")
            .texture_exclusions(
                ["*_disp_*", "*_arm_*", "*_nor_*"]
                    .into_iter()
                    .map(String::from)
                    .collect::<Vec<_>>(),
            )
            // In Wasm, TrenchBroom classes are not automatically registered.
            // So, we need to manually register the classes here
            .register_props()
            .register_proxies()
            .register_class::<Worldspawn>()
            .register_class::<Npc>()
            .register_class::<Player>();
        #[cfg(target_arch = "wasm32")]
        let config = config.no_bsp_lighting(true);
        config
    }));
    app.add_systems(Startup, write_trenchbroom_config);
    app.add_plugins(proxy::plugin);
}

fn write_trenchbroom_config(server: Res<TrenchBroomServer>) {
    #[cfg(target_arch = "wasm32")]
    let _ = server;
    #[cfg(not(target_arch = "wasm32"))]
    {
        info!("Writing TrenchBroom config");
        if let Err(err) = server.config.write_game_config_to_default_directory() {
            error!("Could not write TrenchBroom game config: {err}");
        }
        if let Err(err) = server.config.add_game_to_preferences_in_default_directory() {
            error!("Could not add game to TrenchBroom preferences: {err}");
        }
    }
}

#[derive(SolidClass, Component, Reflect, Default)]
#[reflect(Component)]
#[require(BspWorldspawn)]
#[geometry(GeometryProvider::new().convex_collider().smooth_by_default_angle().with_lightmaps())]
pub(crate) struct Worldspawn;

pub(crate) fn fix_gltf_rotation(mut world: EntityWorldMut) {
    trenchbroom_gltf_rotation_fix(&mut world);
}

pub(crate) trait GetTrenchbroomModelPath: QuakeClass {
    fn scene_path() -> String {
        format!(
            "{file_path}#Scene0",
            file_path = Self::CLASS_INFO.model_path().unwrap()
        )
    }
    fn animation_path(index: u32) -> String {
        format!(
            "{file_path}#Animation{index}",
            file_path = Self::CLASS_INFO.model_path().unwrap()
        )
    }
}

impl<T: QuakeClass> GetTrenchbroomModelPath for T {}

pub(crate) trait LoadTrenchbroomModel {
    fn load_trenchbroom_model<T: QuakeClass>(&self) -> Handle<Scene>;
}

impl LoadTrenchbroomModel for DeferredWorld<'_> {
    fn load_trenchbroom_model<T: QuakeClass>(&self) -> Handle<Scene> {
        self.resource::<AssetServer>().load_trenchbroom_model::<T>()
    }
}

impl LoadTrenchbroomModel for AssetServer {
    fn load_trenchbroom_model<T: QuakeClass>(&self) -> Handle<Scene> {
        self.load(T::scene_path())
    }
}
