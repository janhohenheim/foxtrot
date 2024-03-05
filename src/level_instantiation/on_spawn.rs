use bevy::prelude::*;

pub(crate) use self::{ground::Ground, npc::Npc, player::Player};

mod camera;
mod collider;
mod grass;
mod ground;
mod hidden;
mod npc;
mod orb;
pub(crate) mod player;
mod util;

/// Handles the modifications of objects after they spawn.
/// The reason you will want to do this is that the Blender workflow allows you to add marker components to objects in Blender.
/// These marker components are then used to spawn the rest of the components or modify other existing components in Bevy through code.
pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        grass::plugin,
        ground::plugin,
        camera::plugin,
        orb::plugin,
        player::plugin,
        npc::plugin,
        hidden::plugin,
        collider::plugin,
    ));
}
