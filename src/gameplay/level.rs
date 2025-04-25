//! Spawn the main level.

use bevy::prelude::*;

use crate::{
    asset_tracking::LoadResource, screens::Screen,
    third_party::bevy_trenchbroom::FixTrenchbroomGltfRotation,
};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<LevelAssets>();
    app.register_type::<Level>();
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
        FixTrenchbroomGltfRotation,
    ));
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub(crate) struct Level;

/// A [`Resource`] that contains all the assets needed to spawn the level.
/// We use this to preload assets before the level is spawned.
#[derive(Resource, Asset, Clone, TypePath)]
struct LevelAssets {
    #[dependency]
    level: Handle<Scene>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            //  Run ./scripts/compile_maps.sh and change .map to .bsp when we're done prototyping and want some extra performance
            level: assets.load("maps/foxtrot/foxtrot.map#Scene"),
        }
    }
}
