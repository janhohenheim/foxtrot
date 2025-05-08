//! A loading screen during which game assets are loaded.
//! This reduces stuttering, especially for audio on Wasm.

use bevy::prelude::*;

use crate::{asset_tracking::ResourceHandles, screens::Screen, theme::prelude::*};

use super::LoadingScreen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(LoadingScreen::Assets), spawn_loading_screen);

    app.add_systems(
        Update,
        (
            update_loading_assets_label,
            enter_spawn_level_screen.run_if(all_assets_loaded),
        )
            .chain()
            .run_if(in_state(LoadingScreen::Assets)),
    );

    app.register_type::<LoadingAssetsLabel>();
}

fn spawn_loading_screen(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Loading Screen"),
        StateScoped(LoadingScreen::Assets),
        children![(widget::label("Loading Assets"), LoadingAssetsLabel)],
    ));
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct LoadingAssetsLabel;

fn enter_spawn_level_screen(mut next_screen: ResMut<NextState<LoadingScreen>>) {
    next_screen.set(LoadingScreen::Level);
}

fn all_assets_loaded(resource_handles: Res<ResourceHandles>) -> bool {
    resource_handles.is_all_done()
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
