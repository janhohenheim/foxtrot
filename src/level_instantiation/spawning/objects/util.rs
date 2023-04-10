use bevy::prelude::*;

pub(crate) trait MeshAssetsExt {
    fn get_or_add(&mut self, handle: HandleUntyped, create_mesh: impl Fn() -> Mesh)
        -> Handle<Mesh>;
}

impl MeshAssetsExt for Assets<Mesh> {
    fn get_or_add(
        &mut self,
        handle: HandleUntyped,
        create_mesh: impl Fn() -> Mesh,
    ) -> Handle<Mesh> {
        let handle = handle.typed();
        self.get_or_insert_with(handle.clone_weak(), create_mesh);
        handle
    }
}
