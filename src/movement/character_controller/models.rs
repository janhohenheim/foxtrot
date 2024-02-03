use crate::movement::character_controller::FloatHeight;
use bevy::prelude::*;
use bevy::render::view::NoFrustumCulling;
use bevy_tnua::controller::TnuaController;
use bevy_xpbd_3d::prelude::*;

/// Shift models down because XPBD will make controllers float,
/// but our models definitely should not be floating!
pub(crate) fn offset_models_to_controller(
    mut commands: Commands,
    controllers: Query<(Entity, &Transform, &FloatHeight), (Added<TnuaController>, With<Collider>)>,
    mut transforms: Query<&mut Transform, Without<Collider>>,
    children_q: Query<&Children>,
) {
    for (entity, transform, float_height) in controllers.iter() {
        let offset = (float_height.0 / transform.scale.y) * 2.;
        let children = children_q.get(entity).unwrap();
        for child in children.iter() {
            if let Ok(mut model_transform) = transforms.get_mut(*child) {
                model_transform.translation.y -= offset;
            }
        }

        // Frustum culling is erroneous for animated models because the AABB is not updated.
        commands.entity(entity).insert(NoFrustumCulling);
        for entity in children_q.iter_descendants(entity) {
            commands.entity(entity).insert(NoFrustumCulling);
        }
    }
}
