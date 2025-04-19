use bevy::prelude::*;
use bevy_trenchbroom::config::TrenchBroomConfig;
use generic::*;
use specific::*;

mod effects;
mod generic;
mod specific;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((generic::plugin, specific::plugin, effects::plugin));
}

// We can define a new prop here to make it show up in TrenchBroom.
// Afterwards, we still need to add it to the `LevelAssets` struct to preload it for a given level.

pub(crate) trait RegisterProps {
    fn register_props(self) -> TrenchBroomConfig;
}

impl RegisterProps for TrenchBroomConfig {
    fn register_props(self) -> TrenchBroomConfig {
        self.register_class::<BurningLogs>()
            .register_class::<Grate>()
            .register_class::<Table>()
            .register_class::<Chair>()
            .register_class::<Bookshelf>()
            .register_class::<LampSitting>()
            .register_class::<Crate>()
    }
}

// generic dynamic props

// generic static props

create_prop!(
    Grate,
    "models/darkmod/fireplace/grate.gltf",
    on_add = setup_static_prop_with_convex_hull::<Grate>
);

create_prop!(
    Table,
    "models/darkmod/furniture/tables/rtable1.gltf",
    on_add = setup_static_prop_with_convex_decomposition::<Table>
);

create_prop!(
    Bookshelf,
    "models/darkmod/furniture/shelves/bookshelf02.gltf",
    on_add = setup_static_prop_with_convex_hull::<Bookshelf>
);

// props with a specific setup function

create_prop!(
    LampSitting,
    "models/darkmod/lights/non-extinguishable/round_lantern_sitting.gltf",
    on_add = setup_lamp_sitting
);

create_prop!(
    Chair,
    "models/darkmod/furniture/seating/wchair1.gltf",
    on_add = specific::setup_chair
);

create_prop!(
    BurningLogs,
    "models/darkmod/fireplace/burntwood.gltf",
    on_add = specific::setup_burning_logs
);

create_prop!(
    Crate,
    "models/darkmod/containers/crate01.gltf",
    on_add = setup_crate
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
