use bevy::{
    ecs::component::{ComponentHooks, StorageType},
    prelude::*,
};
use bevy_tnua::prelude::*;
use leafwing_input_manager::prelude::*;

use super::action::CharacterAction;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<InsertCharacterController>();
}

#[derive(Debug, Clone, Copy, Reflect)]
#[reflect(Component)]
struct InsertCharacterController;

impl Component for InsertCharacterController {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(|mut world, entity, _component_id| {
            world
                .commands()
                .entity(entity)
                .insert((
                    TnuaControllerBundle::default(),
                    ActionState::<CharacterAction>::default(),
                ))
                .remove::<InsertCharacterController>();
        });
    }
}
