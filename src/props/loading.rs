use crate::third_party::bevy_trenchbroom::GetTrenchbroomModelPath as _;
use bevy::prelude::*;
use bevy_trenchbroom::class::QuakeClass;

pub(super) fn plugin(_app: &mut App) {}

pub(crate) trait LoadModel {
    fn load_model<T: QuakeClass>(&self) -> Handle<Scene>;
}

impl LoadModel for AssetServer {
    fn load_model<T: QuakeClass>(&self) -> Handle<Scene> {
        self.load(format!("{}#Scene0", T::model_path()))
    }
}
