//! Extension methods and utilities to make using TrenchBroom easier.

use bevy::{ecs::world::DeferredWorld, prelude::*};
use bevy_trenchbroom::{class::QuakeClass, prelude::*};

pub(super) fn plugin(_app: &mut App) {}

#[derive(SolidClass, Component, Reflect, Default)]
#[reflect(Component, QuakeClass)]
#[spawn_hooks(SpawnHooks::new().convex_collider().smooth_by_default_angle())]
pub(crate) struct Worldspawn;

pub(crate) trait GetTrenchbroomModelPath: QuakeClass {
    fn model_path() -> String {
        Self::CLASS_INFO.model_path().unwrap().to_string()
    }
    fn scene_path() -> String {
        format!("{file_path}#Scene0", file_path = Self::model_path())
    }
    fn animation_path(index: u32) -> String {
        format!(
            "{file_path}#Animation{index}",
            file_path = Self::model_path()
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
