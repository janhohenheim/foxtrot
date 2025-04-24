//! Props are generic objects that can be placed in the level. This corresponds to what TrenchBroom calls an "Entity", not to be confused with Bevy's `Entity`.
//! We use this file to define new props and register them with TrenchBroom so that they show up in the level editor.
//! Afterwards, we still need to add new props to the `LevelAssets` struct to preload them for a given level.
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

pub(crate) trait RegisterProps {
    fn register_props(self) -> TrenchBroomConfig;
}

impl RegisterProps for TrenchBroomConfig {
    /// This method is called when the game starts. If we don't target Wasm, we can use the `auto_register` feature of `bevy_trenchbroom` to automatically register all props instead.
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
        #[spawn_hook(bevy_trenchbroom::prelude::preload_model::<Self>)]
        pub(crate) struct $name;
    };
}
// This `use` allows us to use the macro before its definition.
use create_prop;
