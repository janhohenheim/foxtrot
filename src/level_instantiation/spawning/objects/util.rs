use bevy::prelude::*;

pub(crate) trait MeshAssetsExt {
    fn get_or_add(&mut self, handle: Handle<Mesh>, create_mesh: impl Fn() -> Mesh) -> Handle<Mesh>;
}

impl MeshAssetsExt for Assets<Mesh> {
    fn get_or_add(&mut self, handle: Handle<Mesh>, create_mesh: impl Fn() -> Mesh) -> Handle<Mesh> {
        self.get_or_insert_with(handle.clone_weak(), create_mesh);
        handle
    }
}
