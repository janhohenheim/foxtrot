//! Props are generic objects that can be placed in the level. This corresponds to what TrenchBroom calls an "Entity", not to be confused with Bevy's `Entity`.
//! We use this file to define new props and register them with TrenchBroom so that they show up in the level editor.
//! Afterwards, we still need to add new props to the `LevelAssets` struct to preload them for a given level.
use bevy::prelude::*;

mod brush_entity;
mod effects;
mod generic;
mod setup;
mod specific;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        setup::plugin,
        specific::plugin,
        effects::plugin,
        generic::plugin,
        brush_entity::plugin,
    ));
}
