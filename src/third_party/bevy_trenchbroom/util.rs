//! Extension methods and utilities to make using TrenchBroom easier.

use std::f32::consts::FRAC_PI_2;

use bevy::{ecs::world::DeferredWorld, prelude::*};
use bevy_trenchbroom::{bsp::base_classes::BspWorldspawn, class::QuakeClass, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_observer(fix_gltf_rotation);
    app.register_type::<FixTrenchbroomGltfRotation>();
}

#[derive(SolidClass, Component, Reflect, Default)]
#[reflect(Component)]
#[require(BspWorldspawn)]
#[geometry(GeometryProvider::new().convex_collider().smooth_by_default_angle().with_lightmaps())]
pub(crate) struct Worldspawn;

/// TrenchBroom [displays glTFs with a 90 degree rotation](https://github.com/TrenchBroom/TrenchBroom/issues/4447),
/// so we need to replicate that on the scene in order to get the same orientation in our game as in TrenchBroom.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub(crate) struct FixTrenchbroomGltfRotation;

fn fix_gltf_rotation(
    trigger: Trigger<OnAdd, SceneRoot>,
    parents: Query<&Parent>,
    fix_marker: Query<(), With<FixTrenchbroomGltfRotation>>,
    mut transform: Query<&mut Transform>,
) {
    let scene: Entity = trigger.entity();
    for entity in parents.iter_ancestors(scene) {
        if fix_marker.contains(entity) {
            if let Ok(mut transform) = transform.get_mut(scene) {
                transform.rotate_local_y(-FRAC_PI_2);
                break;
            }
        }
    }
}

pub(crate) trait GetTrenchbroomModelPath: QuakeClass {
    fn scene_path() -> String {
        format!(
            "{file_path}#Scene0",
            file_path = Self::CLASS_INFO.model_path().unwrap()
        )
    }
    fn animation_path(index: u32) -> String {
        format!(
            "{file_path}#Animation{index}",
            file_path = Self::CLASS_INFO.model_path().unwrap()
        )
    }
}

impl<T: QuakeClass> GetTrenchbroomModelPath for T {}

pub(crate) trait LoadTrenchbroomModel {
    fn load_trenchbroom_model<T: QuakeClass>(&self) -> Handle<Scene>;
}

impl LoadTrenchbroomModel for DeferredWorld<'_> {
    fn load_trenchbroom_model<T: QuakeClass>(&self) -> Handle<Scene> {
        self.resource::<AssetServer>().load_trenchbroom_model::<T>()
    }
}

impl LoadTrenchbroomModel for AssetServer {
    fn load_trenchbroom_model<T: QuakeClass>(&self) -> Handle<Scene> {
        self.load(T::scene_path())
    }
}
