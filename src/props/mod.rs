//! Props are generic objects that can be placed in the level. This corresponds to what TrenchBroom calls an "Entity", not to be confused with Bevy's `Entity`.
//! We use this file to define new props and register them with TrenchBroom so that they show up in the level editor.
//! Afterwards, we still need to add new props to the `LevelAssets` struct to preload them for a given level.
use bevy::prelude::*;
use bevy_trenchbroom::prelude::*;
use generic::*;

mod effects;
mod generic;
mod specific;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((generic::plugin, specific::plugin, effects::plugin));
    app.add_observer(setup_static_prop_with_convex_hull::<Grate>)
        .add_observer(setup_static_prop_with_convex_decomposition::<Table>)
        .add_observer(setup_static_prop_with_convex_hull::<Bookshelf>);
    app.register_type::<Grate>();
    app.register_type::<Table>();
    app.register_type::<Bookshelf>();
    app.register_type::<Chair>();
    app.register_type::<LampSitting>();
    app.register_type::<LampWallElectric>();
    app.register_type::<Crate>();
}

// generic dynamic props

// generic static props

#[derive(PointClass, Component, Debug, Reflect)]
#[reflect(QuakeClass, Component)]
#[base(Transform, Visibility)]
#[model("models/darkmod/fireplace/grate.gltf")]
#[spawn_hook(preload_model::<Self>)]
pub(crate) struct Grate;

#[derive(PointClass, Component, Debug, Reflect)]
#[reflect(QuakeClass, Component)]
#[base(Transform, Visibility)]
#[model("models/darkmod/furniture/tables/rtable1.gltf")]
#[spawn_hook(preload_model::<Self>)]
pub(crate) struct Table;

#[derive(PointClass, Component, Debug, Reflect)]
#[reflect(QuakeClass, Component)]
#[base(Transform, Visibility)]
#[model("models/darkmod/furniture/shelves/bookshelf02.gltf")]
#[spawn_hook(preload_model::<Self>)]
pub(crate) struct Bookshelf;

// props with a specific setup function

#[derive(PointClass, Component, Debug, Reflect)]
#[reflect(QuakeClass, Component)]
#[base(Transform, Visibility)]
#[model(
    "models/darkmod/lights/non-extinguishable/round_lantern_sitting/round_lantern_sitting.gltf"
)]
#[spawn_hook(preload_model::<Self>)]
pub(crate) struct LampSitting;

#[derive(PointClass, Component, Debug, Reflect)]
#[reflect(QuakeClass, Component)]
#[base(Transform, Visibility)]
#[model(
    "models/darkmod/lights/non-extinguishable/lamp_wall_electric_01/lamp_wall_electric_01.gltf"
)]
#[spawn_hook(preload_model::<Self>)]
#[classname("light_lamp_wall_electric")]
pub(crate) struct LampWallElectric;

#[derive(PointClass, Component, Debug, Reflect)]
#[reflect(QuakeClass, Component)]
#[base(Transform, Visibility)]
#[model("models/darkmod/furniture/seating/wchair1.gltf")]
#[spawn_hook(preload_model::<Self>)]
pub(crate) struct Chair;

#[derive(PointClass, Component, Debug, Reflect)]
#[reflect(QuakeClass, Component)]
#[base(Transform, Visibility)]
#[model("models/darkmod/fireplace/burntwood.gltf")]
#[spawn_hook(preload_model::<Self>)]
pub(crate) struct BurningLogs;

#[derive(PointClass, Component, Debug, Reflect)]
#[reflect(QuakeClass, Component)]
#[base(Transform, Visibility)]
#[model("models/darkmod/containers/crate01.gltf")]
#[spawn_hook(preload_model::<Self>)]
pub(crate) struct Crate;
