use avian_pickup::prop::HeldProp;
use bevy::prelude::*;

mod collision;
mod input;
mod sound;
mod ui;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((input::plugin, ui::plugin, collision::plugin, sound::plugin));
}

pub(crate) fn is_holding_prop(q_prop: Query<&HeldProp>) -> bool {
    !q_prop.is_empty()
}
