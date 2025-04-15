use crate::third_party::bevy_trenchbroom::GetTrenchbroomModelPath as _;
use bevy::prelude::*;
use bevy_trenchbroom::class::QuakeClass;
use dynamic::setup_dynamic_prop;
use util::create_prop;

mod dynamic;
mod specific;
mod util;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((util::plugin, dynamic::plugin, specific::plugin));
}

// We can add a new prop here.
// Afterwards, we still need to add it to the `LevelAssets` struct to preload it for a given level.

pub(crate) fn load_model<T: QuakeClass>(assets: &AssetServer) -> Handle<Scene> {
    assets.load(format!("{}#Scene0", T::model_path()))
}

create_prop!(
    Book,
    "models/book/book.gltf",
    on_add = setup_dynamic_prop::<Book>
);
create_prop!(
    Plate,
    "models/plate/plate.gltf",
    on_add = setup_dynamic_prop::<Plate>
);
create_prop!(
    Mug,
    "models/mug/mug.gltf",
    on_add = setup_dynamic_prop::<Mug>
);
create_prop!(
    CandleUnlit,
    "models/candle_unlit/candle_unlit.gltf",
    on_add = setup_dynamic_prop::<CandleUnlit>
);

create_prop!(
    Candle,
    "models/candle/candle.gltf",
    on_add = specific::setup_candle
);
