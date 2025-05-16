use super::setup::*;
use bevy::prelude::*;
use bevy_trenchbroom::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_observer(setup_static_prop_with_convex_hull::<Grate>)
        .add_observer(setup_static_prop_with_convex_decomposition::<Table>)
        .add_observer(setup_static_prop_with_convex_hull::<Bookshelf>)
        .add_observer(setup_static_prop_with_convex_hull::<Generator2>)
        .add_observer(setup_static_prop_with_convex_hull::<BarrelLargeClosed>)
        .add_observer(setup_static_prop_with_convex_hull::<Barrel01>)
        .add_observer(setup_static_prop_with_convex_hull::<CrateSquare>)
        .add_observer(setup_static_prop_with_convex_hull::<FenceBarsDecorativeSingle>)
        .add_observer(setup_static_prop_with_convex_hull::<DoorStainedGlass>);

    app.add_observer(setup_dynamic_prop_with_convex_hull::<PackageMedium>)
        .add_observer(setup_dynamic_prop_with_convex_hull::<PackageSmall>);

    app.add_observer(setup_nonphysical_prop::<IvyPart8>)
        .add_observer(setup_nonphysical_prop::<SmallDoorSign1>);

    app.register_type::<Grate>();
    app.register_type::<Table>();
    app.register_type::<Bookshelf>();
    app.register_type::<Generator2>();
    app.register_type::<BarrelLargeClosed>();
    app.register_type::<Barrel01>();
    app.register_type::<CrateSquare>();
    app.register_type::<FenceBarsDecorativeSingle>();
    app.register_type::<PackageMedium>();
    app.register_type::<PackageSmall>();
    app.register_type::<DoorStainedGlass>();
    app.register_type::<IvyPart8>();
    app.register_type::<SmallDoorSign1>();
}

// generic dynamic props

#[derive(PointClass, Component, Debug, Reflect)]
#[reflect(QuakeClass, Component)]
#[base(Transform, Visibility)]
#[model("models/darkmod/containers/package_medium.gltf")]
#[spawn_hooks(SpawnHooks::new().preload_model::<Self>())]
pub(crate) struct PackageMedium;

#[derive(PointClass, Component, Debug, Reflect)]
#[reflect(QuakeClass, Component)]
#[base(Transform, Visibility)]
#[model("models/darkmod/containers/package_small.gltf")]
#[spawn_hooks(SpawnHooks::new().preload_model::<Self>())]
pub(crate) struct PackageSmall;

// generic static props

#[derive(PointClass, Component, Debug, Reflect)]
#[reflect(QuakeClass, Component)]
#[base(Transform, Visibility)]
#[model("models/darkmod/fireplace/grate.gltf")]
#[spawn_hooks(SpawnHooks::new().preload_model::<Self>())]
pub(crate) struct Grate;

#[derive(PointClass, Component, Debug, Reflect)]
#[reflect(QuakeClass, Component)]
#[base(Transform, Visibility)]
#[model("models/darkmod/furniture/tables/rtable1.gltf")]
#[spawn_hooks(SpawnHooks::new().preload_model::<Self>())]
pub(crate) struct Table;

#[derive(PointClass, Component, Debug, Reflect)]
#[reflect(QuakeClass, Component)]
#[base(Transform, Visibility)]
#[model("models/darkmod/furniture/shelves/bookshelf02.gltf")]
#[spawn_hooks(SpawnHooks::new().preload_model::<Self>())]
pub(crate) struct Bookshelf;

#[derive(PointClass, Component, Debug, Reflect)]
#[reflect(QuakeClass, Component)]
#[base(Transform, Visibility)]
#[model("models/darkmod/mechanical/generator2/generator2.gltf")]
#[spawn_hooks(SpawnHooks::new().preload_model::<Self>())]
pub(crate) struct Generator2;

#[derive(PointClass, Component, Debug, Reflect)]
#[reflect(QuakeClass, Component)]
#[base(Transform, Visibility)]
#[model("models/darkmod/containers/barrel_large_closed.gltf")]
#[spawn_hooks(SpawnHooks::new().preload_model::<Self>())]
pub(crate) struct BarrelLargeClosed;

#[derive(PointClass, Component, Debug, Reflect)]
#[reflect(QuakeClass, Component)]
#[base(Transform, Visibility)]
#[model("models/darkmod/containers/barrel01.gltf")]
#[spawn_hooks(SpawnHooks::new().preload_model::<Self>())]
pub(crate) struct Barrel01;

#[derive(PointClass, Component, Debug, Reflect)]
#[reflect(QuakeClass, Component)]
#[base(Transform, Visibility)]
#[model("models/darkmod/containers/crate_square.gltf")]
#[spawn_hooks(SpawnHooks::new().preload_model::<Self>())]
pub(crate) struct CrateSquare;

#[derive(PointClass, Component, Debug, Reflect)]
#[reflect(QuakeClass, Component)]
#[base(Transform, Visibility)]
#[model("models/darkmod/architecture/fencing/fence_bars_decorative01_single.gltf")]
#[spawn_hooks(SpawnHooks::new().preload_model::<Self>())]
pub(crate) struct FenceBarsDecorativeSingle;

#[derive(PointClass, Component, Debug, Reflect)]
#[reflect(QuakeClass, Component)]
#[base(Transform, Visibility)]
#[model("models/darkmod/architecture/doors/door_stained_glass_118x52.gltf")]
#[spawn_hooks(SpawnHooks::new().preload_model::<Self>())]
pub(crate) struct DoorStainedGlass;

// Generic non-physical props

#[derive(PointClass, Component, Debug, Reflect)]
#[reflect(QuakeClass, Component)]
#[base(Transform, Visibility)]
#[model("models/darkmod/nature/ivy_part08.gltf")]
#[spawn_hooks(SpawnHooks::new().preload_model::<Self>())]
pub(crate) struct IvyPart8;

#[derive(PointClass, Component, Debug, Reflect)]
#[reflect(QuakeClass, Component)]
#[base(Transform, Visibility)]
#[model("models/darkmod/decorative/signs/small_door_sign1.gltf")]
#[spawn_hooks(SpawnHooks::new().preload_model::<Self>())]
pub(crate) struct SmallDoorSign1;
