use bevy::prelude::*;

mod collision;
mod input;
mod ui;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((input::plugin, ui::plugin, collision::plugin));
}
