use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<DidFixedUpdateHappen>();

    app.add_systems(PreUpdate, reset_did_fixed_update_happen);
    app.add_systems(FixedFirst, set_did_fixed_update_happen);

    app.register_type::<DidFixedUpdateHappen>();
}

fn reset_did_fixed_update_happen(mut did_fixed_update_happen: ResMut<DidFixedUpdateHappen>) {
    **did_fixed_update_happen = false;
}

fn set_did_fixed_update_happen(mut did_fixed_update_happen: ResMut<DidFixedUpdateHappen>) {
    **did_fixed_update_happen = true;
}

/// Stores whether a fixed update happened in the current frame.
#[derive(Resource, Reflect, Default, Debug, Deref, DerefMut)]
#[reflect(Resource)]
pub(crate) struct DidFixedUpdateHappen(pub(crate) bool);

pub(crate) fn did_fixed_update_happen(did_fixed_update_happen: Res<DidFixedUpdateHappen>) -> bool {
    **did_fixed_update_happen
}
