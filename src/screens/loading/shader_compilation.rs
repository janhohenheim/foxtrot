//! A loading screen during which game assets are loaded.
//! This reduces stuttering, especially for audio on Wasm.

use bevy::prelude::*;

use crate::{
    screens::Screen,
    shader_compilation::{LoadedPipelineCount, all_pipelines_loaded, spawn_shader_compilation_map},
    theme::prelude::*,
};

use super::LoadingScreen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(LoadingScreen::Shaders),
        (
            spawn_or_skip_shader_compilation_loading_screen,
            spawn_shader_compilation_map,
        ),
    );

    app.add_systems(
        Update,
        (
            update_loading_shaders_label,
            enter_spawn_level_screen.run_if(all_pipelines_loaded),
        )
            .chain()
            .run_if(in_state(LoadingScreen::Shaders)),
    );

    app.register_type::<LoadingShadersLabel>();
}

fn spawn_or_skip_shader_compilation_loading_screen(
    mut commands: Commands,
    loaded_pipeline_count: Res<LoadedPipelineCount>,
    mut next_screen: ResMut<NextState<LoadingScreen>>,
) {
    if loaded_pipeline_count.is_done() {
        next_screen.set(LoadingScreen::Level);
        return;
    }
    commands.spawn((
        widget::ui_root("Loading Screen"),
        StateScoped(Screen::Loading),
        children![(widget::label("Compiling shaders..."), LoadingShadersLabel)],
    ));
}

fn enter_spawn_level_screen(mut next_screen: ResMut<NextState<LoadingScreen>>) {
    next_screen.set(LoadingScreen::Level);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct LoadingShadersLabel;

fn update_loading_shaders_label(
    mut query: Query<&mut Text, With<LoadingShadersLabel>>,
    loaded_pipeline_count: Res<LoadedPipelineCount>,
) {
    for mut text in query.iter_mut() {
        text.0 = format!(
            "Compiling shaders: {} / {}",
            loaded_pipeline_count.0,
            LoadedPipelineCount::TOTAL_PIPELINES
        );
    }
}
