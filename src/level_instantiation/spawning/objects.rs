pub(crate) mod camera;
pub(crate) mod grass;
pub(crate) mod npc;
pub(crate) mod orb;
pub(crate) mod player;
pub(crate) mod sunlight;
mod util;
use bevy_xpbd_3d::prelude::*;

#[derive(PhysicsLayer)]
pub(crate) enum CollisionLayer {
    Player,
    Character,
    Terrain,
    CameraObstacle,
    Sensor,
}
