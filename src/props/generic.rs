use crate::{asset_tracking::LoadResource, third_party::bevy_trenchbroom::GetTrenchbroomModelPath};

use super::setup::*;
use avian3d::prelude::{ColliderConstructor, RigidBody};
use bevy::prelude::*;
use bevy_trenchbroom::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.load_asset::<Gltf>(PackageMedium::model_path())
        .load_asset::<Gltf>(PackageSmall::model_path())
        .load_asset::<Gltf>(Grate::model_path())
        .load_asset::<Gltf>(Table::model_path())
        .load_asset::<Gltf>(Bookshelf::model_path())
        .load_asset::<Gltf>(Generator2::model_path())
        .load_asset::<Gltf>(BarrelLargeClosed::model_path())
        .load_asset::<Gltf>(Barrel01::model_path())
        .load_asset::<Gltf>(CrateSquare::model_path())
        .load_asset::<Gltf>(FenceBarsDecorativeSingle::model_path())
        .load_asset::<Gltf>(DoorStainedGlass::model_path())
        .load_asset::<Gltf>(IvyPart8::model_path())
        .load_asset::<Gltf>(SmallDoorSign1::model_path());
}

// generic dynamic props

#[point_class(
    base(Transform, Visibility),
    model("models/darkmod/containers/package_medium.gltf")
)]
#[component(on_add = setup_prop::<PackageMedium>(RigidBody::Dynamic, ColliderConstructor::ConvexHullFromMesh))]
pub(crate) struct PackageMedium;

#[point_class(
    base(Transform, Visibility),
    model("models/darkmod/containers/package_small.gltf")
)]
#[component(on_add = setup_prop::<PackageSmall>(RigidBody::Dynamic, ColliderConstructor::ConvexHullFromMesh))]
pub(crate) struct PackageSmall;

// generic static props
#[point_class(
    base(Transform, Visibility),
    model("models/darkmod/fireplace/grate.gltf")
)]
#[component(on_add = setup_prop::<Grate>(RigidBody::Static, ColliderConstructor::ConvexHullFromMesh))]
pub(crate) struct Grate;

#[point_class(
    base(Transform, Visibility),
    model("models/darkmod/furniture/tables/rtable1.gltf")
)]
#[component(on_add = setup_prop::<Table>(RigidBody::Static, ColliderConstructor::ConvexDecompositionFromMesh))]
pub(crate) struct Table;

#[point_class(
    base(Transform, Visibility),
    model("models/darkmod/furniture/shelves/bookshelf02.gltf")
)]
#[component(on_add = setup_prop::<Bookshelf>(RigidBody::Static, ColliderConstructor::ConvexHullFromMesh))]
pub(crate) struct Bookshelf;

#[point_class(
    base(Transform, Visibility),
    model("models/darkmod/mechanical/generator2/generator2.gltf")
)]
#[component(on_add = setup_prop::<Generator2>(RigidBody::Static, ColliderConstructor::ConvexHullFromMesh))]
pub(crate) struct Generator2;

#[point_class(
    base(Transform, Visibility),
    model("models/darkmod/containers/barrel_large_closed.gltf")
)]
#[component(on_add = setup_prop::<BarrelLargeClosed>(RigidBody::Static, ColliderConstructor::ConvexHullFromMesh))]
pub(crate) struct BarrelLargeClosed;

#[point_class(
    base(Transform, Visibility),
    model("models/darkmod/containers/barrel01.gltf")
)]
#[component(on_add = setup_prop::<Barrel01>(RigidBody::Static, ColliderConstructor::ConvexHullFromMesh))]
pub(crate) struct Barrel01;

#[point_class(
    base(Transform, Visibility),
    model("models/darkmod/containers/crate_square.gltf")
)]
#[component(on_add = setup_prop::<CrateSquare>(RigidBody::Static, ColliderConstructor::ConvexHullFromMesh))]
pub(crate) struct CrateSquare;

#[point_class(
    base(Transform, Visibility),
    model("models/darkmod/architecture/fencing/fence_bars_decorative01_single.gltf")
)]
#[component(on_add = setup_prop::<FenceBarsDecorativeSingle>(RigidBody::Static, ColliderConstructor::ConvexHullFromMesh))]
pub(crate) struct FenceBarsDecorativeSingle;

#[point_class(
    base(Transform, Visibility),
    model("models/darkmod/architecture/doors/door_stained_glass_118x52.gltf")
)]
#[component(on_add = setup_prop::<DoorStainedGlass>(RigidBody::Static, ColliderConstructor::ConvexHullFromMesh))]
pub(crate) struct DoorStainedGlass;

// Generic non-physical props

#[point_class(
    base(Transform, Visibility),
    model("models/darkmod/nature/ivy_part08.gltf")
)]
#[component(on_add = setup_nonphysical_prop::<IvyPart8>)]
pub(crate) struct IvyPart8;

#[point_class(
    base(Transform, Visibility),
    model("models/darkmod/decorative/signs/small_door_sign1.gltf")
)]
#[component(on_add = setup_nonphysical_prop::<SmallDoorSign1>)]
pub(crate) struct SmallDoorSign1;
