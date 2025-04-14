//! Spawn the main level.

use assets::LevelAssets;
use bevy::{prelude::*, scene::SceneInstanceReady};

use crate::screens::Screen;

mod assets;
pub(crate) mod dynamic_props;
pub(crate) mod prop_util;
pub(crate) mod specific_props;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Level>();
    app.add_plugins((
        prop_util::plugin,
        dynamic_props::plugin,
        specific_props::plugin,
        assets::plugin,
    ));
}

/// A [`Command`] to spawn the level.
/// Functions that accept only `&mut World` as their parameter implement [`Command`].
/// We use this style when a command requires no configuration.
pub(crate) fn spawn_level(world: &mut World) {
    let assets = world.resource::<LevelAssets>();
    world
        .spawn((
            Name::new("Level"),
            SceneRoot(assets.level.clone()),
            StateScoped(Screen::Gameplay),
            Level,
        ))
        .observe(advance_to_gameplay_screen);
}

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub(crate) struct Level;

fn advance_to_gameplay_screen(
    _trigger: Trigger<SceneInstanceReady>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    next_screen.set(Screen::Gameplay);
}
