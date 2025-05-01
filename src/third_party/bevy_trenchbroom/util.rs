//! Extension methods and utilities to make using TrenchBroom easier.

use bevy::{ecs::world::DeferredWorld, prelude::*};
use bevy_trenchbroom::{
    class::{QuakeClass, QuakeClassSpawnView},
    prelude::*,
};

pub(super) fn plugin(_app: &mut App) {}

#[derive(SolidClass, Component, Reflect, Default)]
#[reflect(Component)]
#[geometry(GeometryProvider::new().convex_collider().smooth_by_default_angle().with_lightmaps())]
pub(crate) struct Worldspawn;

pub(crate) trait GetTrenchbroomModelPath: QuakeClass {
    fn ktx_model_path() -> String {
        Self::CLASS_INFO
            .model_path()
            .unwrap()
            .replace(".gltf", "_ktx2.gltf")
    }
    fn scene_path() -> String {
        format!("{file_path}#Scene0", file_path = Self::ktx_model_path())
    }
    fn animation_path(index: u32) -> String {
        format!(
            "{file_path}#Animation{index}",
            file_path = Self::ktx_model_path()
        )
    }
}

#[track_caller]
pub(crate) fn preload_ktx_model<T: QuakeClass>(
    view: &mut QuakeClassSpawnView,
) -> anyhow::Result<()> {
    let handle = view
        .load_context
        .loader()
        .with_unknown_type()
        .load(T::ktx_model_path());
    view.preload_asset(handle.untyped());
    Ok(())
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
