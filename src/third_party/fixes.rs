use bevy::prelude::*;
use bevy_fix_cursor_unlock_web::FixPointerUnlockPlugin;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(FixPointerUnlockPlugin);
}
