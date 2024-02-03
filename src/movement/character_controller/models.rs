use bevy::prelude::*;
use bevy_tnua::controller::TnuaController;
use bevy_xpbd_3d::prelude::*;

pub(crate) fn offset_models_to_character_controller(
    controllers: Query<(&Children, &Collider, &Transform), Added<TnuaController>>,
    mut transforms: Query<&mut Transform, Without<TnuaController>>,
) {
    for (children, collider, transform) in controllers.iter() {
        let aabb = collider.shape_scaled().compute_local_aabb();
        let height = (aabb.maxs.y - aabb.mins.y) * transform.scale.y;
        for child in children.iter() {
            let mut transform = transforms.get_mut(*child).unwrap();
            transform.translation.y -= height;
        }
    }
}
