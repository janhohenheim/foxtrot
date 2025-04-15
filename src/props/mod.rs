use bevy::prelude::*;
use dynamic::setup_dynamic_prop;
use macro_impl::create_prop;

mod dynamic;
pub(crate) mod loading;
mod specific;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((loading::plugin, dynamic::plugin, specific::plugin));
}

// We can add a new prop here.
// Afterwards, we still need to add it to the `LevelAssets` struct to preload it for a given level.

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

// This is nested in a module so that Rust allows us to define it at the end of the file.
mod macro_impl {
    macro_rules! create_prop {
        ($name:ident, $model:expr, on_add = $on_add:ty) => {
            #[derive(
                bevy_trenchbroom::prelude::PointClass,
                Component,
                Debug,
                Clone,
                Copy,
                PartialEq,
                Eq,
                Default,
                Reflect,
            )]
            #[reflect(Component)]
            #[require(Transform, Visibility)]
            #[model($model)]
            #[component(on_add = $on_add)]
            pub(crate) struct $name;
        };
    }
    pub(super) use create_prop;
}
