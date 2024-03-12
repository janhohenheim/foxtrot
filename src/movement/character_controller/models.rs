use crate::movement::character_controller::FloatHeight;
use crate::GameState;
use bevy::transform::TransformSystem;
use bevy::{prelude::*, render::view::NoFrustumCulling};
use bevy_tnua::controller::TnuaController;
use bevy_xpbd_3d::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        prepare_models_of_controllers
            .after(PhysicsSet::Sync)
            .before(TransformSystem::TransformPropagate)
            .run_if(in_state(GameState::Playing)),
    );
}

fn prepare_models_of_controllers(
    mut commands: Commands,
    controllers: Query<(Entity, &Transform, &FloatHeight), (Added<TnuaController>, With<Collider>)>,
    mut transforms: Query<&mut Transform, Without<Collider>>,
    children_q: Query<&Children>,
    meshes: Query<&Handle<Mesh>>,
) {
    for (entity, transform, float_height) in controllers.iter() {
        // Shift models down because Tnua will make controllers float,
        // but our models definitely should not be floating!
        let offset = (float_height.0 / transform.scale.y) * 2.;
        let children = children_q.get(entity).unwrap();
        for child in children.iter() {
            if let Ok(mut model_transform) = transforms.get_mut(*child) {
                model_transform.translation.y -= offset;
            }
        }

        // Frustum culling is erroneous for animated models because the AABB can be too small
        for entity in children_q.iter_descendants(entity) {
            if meshes.contains(entity) {
                commands.entity(entity).insert(NoFrustumCulling);
            }
        }
    }
}
