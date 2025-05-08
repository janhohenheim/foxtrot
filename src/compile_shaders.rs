use bevy::prelude::*;

use bevy::prelude::*;
use bevy_trenchbroom::util::DoNotFixGltfRotationsUnderMe;

use crate::asset_tracking::LoadResource as _;
use crate::screens::loading::LoadingScreen;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<CompileShadersAssets>();
}

pub(crate) fn spawn_compile_shaders_map(
    mut commands: Commands,
    compile_shaders_assets: Res<CompileShadersAssets>,
) {
    commands.spawn((
        Name::new("Compile Shaders Map"),
        SceneRoot(compile_shaders_assets.level.clone()),
        StateScoped(LoadingScreen::Shaders),
        // We already fix the coordinate system for all glTFs in the app,
        // so we opt out of bevy_trenchbroom's coordinate system fixing.
        DoNotFixGltfRotationsUnderMe,
    ));
}

/// A [`Resource`] that contains all the assets needed to spawn the level.
/// We use this to preload assets before the level is spawned.
#[derive(Resource, Asset, Clone, TypePath)]
pub(crate) struct CompileShadersAssets {
    #[dependency]
    pub(crate) level: Handle<Scene>,
}

impl FromWorld for CompileShadersAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();

        Self {
            // This map just loads all effects at once to try to force shader compilation.
            level: assets.load("maps/compile_shaders/compile_shaders.map#Scene"),
        }
    }
}
