//! A loading screen during which game assets are loaded.
//! This reduces stuttering, especially for audio on Wasm.

use bevy::prelude::*;

use crate::{
    asset_tracking::{ResourceHandles, all_assets_loaded},
    theme::prelude::*,
};

use super::LoadingScreen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(LoadingScreen::Assets),
        spawn_or_skip_asset_loading_screen,
    );

    app.add_systems(
        Update,
        (
            update_loading_assets_label,
            enter_compile_shader_screen.run_if(all_assets_loaded),
        )
            .chain()
            .run_if(in_state(LoadingScreen::Assets)),
    );

    app.register_type::<LoadingAssetsLabel>();
}

fn spawn_or_skip_asset_loading_screen(
    mut commands: Commands,
    resource_handles: Res<ResourceHandles>,
    mut next_screen: ResMut<NextState<LoadingScreen>>,
) {
    if resource_handles.is_all_done() {
        next_screen.set(LoadingScreen::Shaders);
        return;
    }
    commands.spawn((
        widget::ui_root("Loading Screen"),
        StateScoped(LoadingScreen::Assets),
        children![(widget::label("Loading Assets"), LoadingAssetsLabel)],
    ));
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct LoadingAssetsLabel;

fn enter_compile_shader_screen(mut next_screen: ResMut<NextState<LoadingScreen>>) {
    next_screen.set(LoadingScreen::Shaders);
}

fn update_loading_assets_label(
    mut query: Query<&mut Text, With<LoadingAssetsLabel>>,
    resource_handles: Res<ResourceHandles>,
) {
    for mut text in query.iter_mut() {
        text.0 = format!(
            "Loading Assets: {} / {}",
            resource_handles.finished_count(),
            resource_handles.total_count()
        );
    }
}
