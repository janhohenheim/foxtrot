use bevy::prelude::*;
#[cfg(feature = "hot_patch")]
use bevy_simple_subsecond_system::hot;

use bevy::render::render_resource::{CachedPipelineState, PipelineCache};
use bevy::render::{MainWorld, RenderApp};

use crate::asset_tracking::LoadResource as _;
use crate::screens::loading::LoadingScreen;

pub(super) fn plugin(app: &mut App) {
    app.load_resource::<CompileShadersAssets>();

    app.init_resource::<LoadedPipelineCount>();

    app.sub_app_mut(RenderApp)
        .add_systems(ExtractSchedule, update_loaded_pipeline_count);

    app.register_type::<LoadedPipelineCount>();
}

#[cfg_attr(feature = "hot_patch", hot)]
pub(crate) fn spawn_shader_compilation_map(
    mut commands: Commands,
    compile_shaders_assets: Res<CompileShadersAssets>,
) {
    commands.spawn((
        Name::new("Compile Shaders Map"),
        SceneRoot(compile_shaders_assets.level.clone()),
        StateScoped(LoadingScreen::Shaders),
    ));
}

/// A [`Resource`] that contains all the assets needed to spawn the level.
/// We use this to preload assets before the level is spawned.
#[derive(Resource, Asset, Clone, TypePath)]
pub(crate) struct CompileShadersAssets {
    #[dependency]
    level: Handle<Scene>,
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

/// A `Resource` in the main world that stores the number of pipelines that are ready.
#[derive(Resource, Default, Debug, Deref, DerefMut, Reflect)]
#[reflect(Resource)]
pub(crate) struct LoadedPipelineCount(pub(crate) usize);

impl LoadedPipelineCount {
    pub(crate) fn is_done(&self) -> bool {
        self.0 >= Self::TOTAL_PIPELINES
    }

    /// These numbers have to be tuned by hand, unfortunately.
    /// When in doubt, better stay a bit too low, or the player won't advance past the loading screen.
    pub(crate) const TOTAL_PIPELINES: usize = {
        #[cfg(feature = "native")]
        {
            #[cfg(feature = "dev")]
            {
                63
            }
            #[cfg(not(feature = "dev"))]
            {
                62
            }
        }
        #[cfg(not(feature = "native"))]
        {
            #[cfg(feature = "dev")]
            {
                24
            }
            #[cfg(not(feature = "dev"))]
            {
                23
            }
        }
    };
}

#[cfg_attr(feature = "hot_patch", hot)]
fn update_loaded_pipeline_count(mut main_world: ResMut<MainWorld>, cache: Res<PipelineCache>) {
    if let Some(mut pipelines_ready) = main_world.get_resource_mut::<LoadedPipelineCount>() {
        let count = cache
            .pipelines()
            .filter(|pipeline| matches!(pipeline.state, CachedPipelineState::Ok(_)))
            .count();

        if pipelines_ready.0 == count {
            return;
        }

        pipelines_ready.0 = count;
    }
}

pub(crate) fn all_pipelines_loaded(loaded_pipeline_count: Res<LoadedPipelineCount>) -> bool {
    loaded_pipeline_count.is_done()
}
