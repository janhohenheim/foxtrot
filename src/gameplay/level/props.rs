use bevy::prelude::*;

use super::dynamic_props::setup_dynamic_prop;
use super::prop_util::create_prop;
use super::specific_props;

pub(super) fn plugin(_app: &mut App) {}

// We can add a new prop here.
// Afterwards, we still need to add its assets to the `LevelAssets` struct
// and the `model_for_class` method.

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
    on_add = specific_props::setup_candle
);
