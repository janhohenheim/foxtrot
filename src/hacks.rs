use avian3d::prelude::*;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.observe(fix_collider);
}

fn fix_collider(
    trigger: Trigger<OnAdd, Collider>,
    mut q_floor: Query<(&GlobalTransform, &mut Collider)>,
) {
    let Ok((transform, mut collider)) = q_floor.get_mut(trigger.entity()) else {
        return;
    };
    let transform = transform.compute_transform();
    collider.set_scale(transform.scale, 10);
}
