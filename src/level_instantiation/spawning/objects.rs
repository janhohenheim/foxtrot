use bevy_rapier3d::prelude::*;
use bitflags::bitflags;

pub(crate) mod camera;
pub(crate) mod level;
pub(crate) mod npc;
pub(crate) mod orb;
pub(crate) mod player;
pub(crate) mod point_light;
pub(crate) mod primitives;
pub(crate) mod skydome;
pub(crate) mod sunlight;
mod util;

bitflags! {
    pub(crate) struct GameCollisionGroup: u32 {
        const PLAYER = 1 << 0;
        const OTHER = 1 << 31;

        const ALL = u32::MAX;
        const NONE = 0;
    }
}

impl From<GameCollisionGroup> for Group {
    fn from(value: GameCollisionGroup) -> Self {
        // Both are u32, so this will never panic.
        Self::from_bits(value.bits()).expect("Failed to convert GameCollisionGroup to rapier Group")
    }
}
