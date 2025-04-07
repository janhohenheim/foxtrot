use bevy::prelude::*;
use bevy_trenchbroom::{bsp::base_classes::BspWorldspawn, class::QuakeClass, prelude::*};

mod proxy;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(TrenchBroomPlugin(
        TrenchBroomConfig::new("foxtrot").texture_exclusions(
            ["*_disp_*", "*_arm_*", "*_nor_*"]
                .into_iter()
                .map(String::from)
                .collect::<Vec<_>>(),
        ),
    ));
    app.add_systems(Startup, write_trenchbroom_config);
    app.add_plugins(proxy::plugin);
}

fn write_trenchbroom_config(server: Res<TrenchBroomServer>) {
    info!("Writing TrenchBroom config");
    if let Err(err) = server.config.write_to_default_folder() {
        error!("Could not write TrenchBroom config: {err}");
    }
}

#[derive(SolidClass, Component, Reflect, Default)]
#[reflect(Component)]
#[require(BspWorldspawn)]
#[geometry(GeometryProvider::new().convex_collider().smooth_by_default_angle())]
struct Worldspawn;

pub(crate) trait LoadTrenchbroomModel {
    fn load_trenchbroom_model<T: QuakeClass>(&self) -> Handle<Scene>;
}

impl LoadTrenchbroomModel for AssetServer {
    fn load_trenchbroom_model<T: QuakeClass>(&self) -> Handle<Scene> {
        let model = T::CLASS_INFO.model.unwrap().trim_matches('"');
        self.load(format!("{model}#Scene0"))
    }
}
