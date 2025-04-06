use bevy::prelude::*;
use bevy_trenchbroom::{bsp::base_classes::BspWorldspawn, class::QuakeClass, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(TrenchBroomPlugin(TrenchBroomConfig::new("foxtrot")))
        .add_systems(Startup, write_trenchbroom_config);
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

pub trait LoadTrenchbroomModel {
    fn load_trenchbroom_model<T: QuakeClass>(&self) -> Handle<Scene>;
}

impl LoadTrenchbroomModel for AssetServer {
    fn load_trenchbroom_model<T: QuakeClass>(&self) -> Handle<Scene> {
        let model = T::CLASS_INFO.model.unwrap().trim_matches('"');
        self.load(format!("{model}#Scene0"))
    }
}
