//! Spawn the main level.

use bevy::prelude::*;

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
            // Use .map for dev and non-native builds
            #[cfg(any(feature = "dev", not(feature = "native")))]
            level: assets.load("maps/foxtrot/foxtrot.map#Scene"),
            // Use .bsp for native release builds
            //  Run ./scripts/compile_maps.sh when we're done prototyping and want some extra performance
            #[cfg(all(feature = "native", not(feature = "dev")))]
            level: assets.load("maps/foxtrot/foxtrot.bsp#Scene"),
        }
    }
}
