use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_tnua::{TnuaNotPlatform, prelude::*};
use bevy_tnua_avian3d::*;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        TnuaControllerPlugin::new(FixedUpdate),
        TnuaAvian3dPlugin::new(FixedUpdate),
    ));
    app.add_observer(copy_not_platform_marker_from_rigid_body_to_colliders);
}

fn copy_not_platform_marker_from_rigid_body_to_colliders(
    trigger: Trigger<OnAdd, ColliderParent>,
    mut commands: Commands,
    q_collider_parent: Query<&ColliderParent>,
    q_tnua_not_platform: Query<&TnuaNotPlatform>,
) {
    let parent = q_collider_parent.get(trigger.entity()).unwrap();
    if !q_tnua_not_platform.contains(parent.get()) {
        return;
    }
    commands.entity(trigger.entity()).insert(TnuaNotPlatform);
}
