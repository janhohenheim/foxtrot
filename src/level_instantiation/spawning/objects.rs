use bevy_rapier3d::prelude::*;
use bitflags::bitflags;

pub mod camera;
pub mod level;
pub mod npc;
pub mod orb;
pub mod player;
pub mod point_light;
pub mod primitives;
pub mod skydome;
pub mod sunlight;

bitflags! {
    pub struct GameCollisionGroup: u32 {
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
