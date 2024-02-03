use crate::movement::character_controller::FloatHeight;
use bevy::prelude::*;
use bevy_tnua::controller::TnuaController;
use bevy_xpbd_3d::prelude::*;

/// Shift models down because XPBD will make controllers float,
/// but our models definitely should not be floating!
pub(crate) fn offset_models_to_controller(
    controllers: Query<
        (&Transform, &FloatHeight, &Children),
        (Added<TnuaController>, With<Collider>),
    >,
    mut transforms: Query<&mut Transform, Without<Collider>>,
) {
    for (transform, float_height, children) in controllers.iter() {
        let offset = (float_height.0 / transform.scale.y) * 2.;
        for child in children.iter() {
            if let Ok(mut model_transform) = transforms.get_mut(*child) {
                model_transform.translation.y -= offset;
            }
        }
    }
}
