//! Spawn the main level.

use assets::LevelAssets;
use bevy::prelude::*;

use crate::screens::Screen;

mod assets;
mod props;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Level>();
    app.add_plugins((props::plugin, assets::plugin));
}

/// A [`Command`] to spawn the level.
/// Functions that accept only `&mut World` as their parameter implement [`Command`].
/// We use this style when a command requires no configuration.
pub(crate) fn spawn_level(world: &mut World) {
    let assets = world.resource::<LevelAssets>();
    world.spawn((
        Name::new("Level"),
        SceneRoot(assets.level.clone()),
        StateScoped(Screen::Gameplay),
        Level,
    ));
}

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub(crate) struct Level;
