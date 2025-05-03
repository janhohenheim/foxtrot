//! Spawn the main level.

use bevy::prelude::*;
use bevy_trenchbroom::util::DoNotFixGltfRotationsUnderMe;

use crate::{asset_tracking::LoadResource, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<LevelAssets>();
    app.register_type::<Level>();
}

/// A [`Command`] to spawn the level.
/// Functions that accept only `&mut World` as their parameter implement [`Command`].
/// We use this style when a command requires no configuration.
pub(crate) fn spawn_level(mut commands: Commands, level_assets: Res<LevelAssets>) {
    commands.spawn((
        Name::new("Level"),
        SceneRoot(level_assets.level.clone()),
        StateScoped(Screen::Gameplay),
        Level,
        // We already fix the coordinate system for all glTFs in the app,
        // so we opt out of bevy_trenchbroom's coordinate system fixing.
        DoNotFixGltfRotationsUnderMe,
    ));
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub(crate) struct Level;

/// A [`Resource`] that contains all the assets needed to spawn the level.
/// We use this to preload assets before the level is spawned.
#[derive(Resource, Asset, Clone, TypePath)]
pub(crate) struct LevelAssets {
    #[dependency]
    pub(crate) level: Handle<Scene>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();

        Self {
            // Use .map for dev builds
            #[cfg(feature = "dev")]
            level: assets.load("maps/foxtrot/foxtrot.bsp#Scene"),
            // We can generate .bsp files from .map files with ./scripts/compile_maps.sh
            // when we're done prototyping and want some extra performance and lightmaps
            #[cfg(not(feature = "dev"))]
            level: assets.load("maps/foxtrot/foxtrot.bsp#Scene"),
        }
    }
}
