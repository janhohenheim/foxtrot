use std::any::TypeId;

use bevy::prelude::*;
use bevy_trenchbroom::class::QuakeClass;

use crate::{
    asset_tracking::LoadResource, third_party::bevy_trenchbroom::GetTrenchbroomModelPath as _,
};

use super::props::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<LevelAssets>();
    app.load_resource::<LevelAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub(crate) struct LevelAssets {
    #[dependency]
    pub(crate) level: Handle<Scene>,
    #[dependency]
    pub(crate) book_model: Handle<Scene>,
    #[dependency]
    pub(crate) plate_model: Handle<Scene>,
    #[dependency]
    pub(crate) mug_model: Handle<Scene>,
    #[dependency]
    pub(crate) candle_unlit_model: Handle<Scene>,
    #[dependency]
    pub(crate) candle_model: Handle<Scene>,
}

impl LevelAssets {
    pub(crate) fn model_for_class<T: QuakeClass>(&self) -> Handle<Scene> {
        let type_id = TypeId::of::<T>();
        if type_id == TypeId::of::<Book>() {
            self.book_model.clone()
        } else if type_id == TypeId::of::<Plate>() {
            self.plate_model.clone()
        } else if type_id == TypeId::of::<Mug>() {
            self.mug_model.clone()
        } else if type_id == TypeId::of::<CandleUnlit>() {
            self.candle_unlit_model.clone()
        } else if type_id == TypeId::of::<Candle>() {
            self.candle_model.clone()
        } else {
            panic!(
                "No model for class with type id: {:?}. Did you forget to add it to the `LevelAssets`?",
                type_id
            )
        }
    }
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            //  Run ./scripts/compile_maps.sh and change .map to .bsp when you're done prototyping and want some extra performance
            level: assets.load("maps/foxtrot/foxtrot.map#Scene"),
            book_model: load_model::<Book>(assets),
            plate_model: load_model::<Plate>(assets),
            mug_model: load_model::<Mug>(assets),
            candle_unlit_model: load_model::<CandleUnlit>(assets),
            candle_model: load_model::<Candle>(assets),
        }
    }
}

fn load_model<T: QuakeClass>(assets: &AssetServer) -> Handle<Scene> {
    assets.load(format!("{}#Scene0", T::model_path()))
}
