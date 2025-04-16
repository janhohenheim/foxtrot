use bevy::prelude::*;
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

pub(crate) trait GetTrenchbroomModelPath {
    fn file_path() -> String;
    fn scene_path() -> String {
        format!("{file_path}#Scene0", file_path = Self::file_path())
    }
    fn animation_path(index: u32) -> String {
        format!(
            "{file_path}#Animation{index}",
            file_path = Self::file_path()
        )
    }
}

impl<T: QuakeClass> GetTrenchbroomModelPath for T {
    fn file_path() -> String {
        Self::CLASS_INFO
            .model
            .unwrap()
            .trim_matches('"')
            .to_string()
    }
}
