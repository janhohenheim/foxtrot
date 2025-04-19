use bevy::prelude::*;
use bevy_trenchbroom::config::TrenchBroomConfig;
use generic::{setup_dynamic_prop, setup_static_prop};

mod generic;
mod specific;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((generic::plugin, specific::plugin));
}

// We can define a new prop here to make it show up in TrenchBroom.
// Afterwards, we still need to add it to the `LevelAssets` struct to preload it for a given level.

pub(crate) trait RegisterProps {
    fn register_props(self) -> TrenchBroomConfig;
}

impl RegisterProps for TrenchBroomConfig {
    fn register_props(self) -> TrenchBroomConfig {
        self.register_class::<Book>()
            .register_class::<Plate>()
            .register_class::<Mug>()
            .register_class::<CandleUnlit>()
            .register_class::<Candle>()
            .register_class::<Drawers>()
            .register_class::<BurningLogs>()
            .register_class::<Grate>()
    }
}

// generic dynamic props

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

// generic static props

create_prop!(
    Drawers,
    "models/drawers/drawers.gltf",
    on_add = setup_static_prop::<Drawers>
);
create_prop!(
    Grate,
    "models/grate/grate.gltf",
    on_add = setup_static_prop::<Grate>
);

// props with a specific setup function

create_prop!(
    Candle,
    "models/candle/candle.gltf",
    on_add = specific::setup_candle
);

create_prop!(
    BurningLogs,
    "models/burning_logs/burning_logs.gltf",
    on_add = specific::setup_burning_logs
);

// This macro does nothing fancy, it's just here to save us some boilerplate when defining new prop classes :)
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
        #[base(Transform, Visibility)]
        #[model($model)]
        #[component(on_add = $on_add)]
        pub(crate) struct $name;
    };
}
// This `use` allows us to use the macro before its definition.
use create_prop;
