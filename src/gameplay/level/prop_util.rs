use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_tnua::TnuaNotPlatform;


pub(super) fn plugin(app: &mut App) {
    app.add_observer(add_not_platform_to_props);
}

macro_rules! create_prop {
    ($name:ident, $model:expr) => {
        create_prop!($name, $model, on_add = setup_dynamic_prop::<$name>);
    };
    ($name:ident, $model:expr, on_add = $on_add:ty) => {
        #[derive(PointClass, Component, Debug, Clone, Copy, PartialEq, Eq, Default, Reflect)]
        #[reflect(Component)]
        #[require(Transform, Visibility)]
        #[model($model)]
        #[component(on_add = $on_add)]
        pub(crate) struct $name;
    };
}
pub(crate) use create_prop;

fn add_not_platform_to_props(
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
