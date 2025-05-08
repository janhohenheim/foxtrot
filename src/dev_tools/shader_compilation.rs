use bevy::prelude::*;

use crate::shader_compilation::LoadedPipelineCount;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, print_more_pipelines_than_expected);
}

fn print_more_pipelines_than_expected(loaded_pipeline_count: Res<LoadedPipelineCount>) {
    if loaded_pipeline_count.is_changed()
        && loaded_pipeline_count.0 > LoadedPipelineCount::TOTAL_PIPELINES
    {
        warn!(
            "Loaded more pipelines than expected: {} / {}. Bump the expected number of pipelines in `LoadedPipelineCount::TOTAL_PIPELINES`.",
            loaded_pipeline_count.0,
            LoadedPipelineCount::TOTAL_PIPELINES
        );
    }
}
