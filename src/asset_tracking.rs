//! A high-level way to load collections of asset handles as resources.

use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<ResourceHandles>();
    app.add_systems(PreUpdate, load_resource_assets);
}

pub trait LoadResource {
    /// This will load the [`Resource`] as an [`Asset`]. When all of its asset dependencies
    /// have been loaded, it will be inserted as a resource. This ensures that the resource only
    /// exists when the assets are ready.
    fn load_resource<T: Resource + Asset + Clone + FromWorld>(&mut self) -> &mut Self;
}

impl LoadResource for App {
    fn load_resource<T: Resource + Asset + Clone + FromWorld>(&mut self) -> &mut Self {
        self.init_asset::<T>();
        let world = self.world_mut();
        let value = T::from_world(world);
        let assets = world.resource::<AssetServer>();
        let handle = assets.add(value);
        let mut handles = world.resource_mut::<ResourceHandles>();
        handles.waiting.push((handle.untyped(), |world, handle| {
            let assets = world.resource::<Assets<T>>();
            if let Some(value) = assets.get(handle.id().typed::<T>()) {
                world.insert_resource(value.clone());
            }
        }));
        self
    }
}

/// A function that inserts a loaded resource.
type InsertLoadedResource = fn(&mut World, &UntypedHandle);

#[derive(Resource, Default)]
struct ResourceHandles {
    waiting: Vec<(UntypedHandle, InsertLoadedResource)>,
    finished: Vec<UntypedHandle>,
}

fn load_resource_assets(world: &mut World) {
    world.resource_scope(|world, mut resource_handles: Mut<ResourceHandles>| {
        world.resource_scope(|world, assets: Mut<AssetServer>| {
            for _ in 0..resource_handles.waiting.len() {
                let (handle, insert_fn) = resource_handles.waiting.pop().unwrap();
                if assets.is_loaded_with_dependencies(&handle) {
                    insert_fn(world, &handle);
                    resource_handles.finished.push(handle);
                } else {
                    resource_handles.waiting.push((handle, insert_fn));
                }
            }
        });
    });
}
