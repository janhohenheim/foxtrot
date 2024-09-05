//! Spawn the main level.

use bevy::{color::palettes::tailwind, ecs::world::Command, prelude::*};
use blenvy::*;

use crate::{demo::player::SpawnPlayer, screens::Screen};

pub(super) fn plugin(_app: &mut App) {
    // No setup required for this plugin.
    // It's still good to have a function here so that we can add some setup
    // later if needed.
}

/// A [`Command`] to spawn the level.
/// Functions that accept only `&mut World` as their parameter implement [`Command`].
/// We use this style when a command requires no configuration.
pub fn spawn_level(world: &mut World) {
    world.spawn((
        Name::new("Level"),
        BlueprintInfo::from_path("levels/World.glb"),
        SpawnBlueprint,
        HideUntilReady,
        GameWorldTag,
        StateScoped(Screen::Gameplay),
    ));
    world.insert_resource(AmbientLight {
        color: tailwind::SKY_100.into(),
        brightness: 400.0,
    });
    SpawnPlayer::default().apply(world);
}
