use bevy::prelude::*;
use bevy_fix_cursor_unlock_web::FixPointerUnlockPlugin;
use bevy_fix_gltf_coordinate_system::FixGltfCoordinateSystemPlugin;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((FixGltfCoordinateSystemPlugin, FixPointerUnlockPlugin));
}
