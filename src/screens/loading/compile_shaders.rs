//! A loading screen during which game assets are loaded.
//! This reduces stuttering, especially for audio on Wasm.

use bevy::prelude::*;

use crate::{
    asset_tracking::ResourceHandles, compile_shaders::spawn_compile_shaders_map, screens::Screen,
    theme::prelude::*,
};

use super::LoadingScreen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(LoadingScreen::Shaders),
        (spawn_loading_screen, spawn_compile_shaders_map),
    );

    app.add_systems(
        Update,
        enter_spawn_level_screen.run_if(in_state(LoadingScreen::Shaders).and(all_shaders_compiled)),
    );
}

fn spawn_loading_screen(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Loading Screen"),
        StateScoped(Screen::Loading),
        children![widget::label("Compiling shaders...")],
    ));
}

fn enter_spawn_level_screen(mut next_screen: ResMut<NextState<LoadingScreen>>) {
    next_screen.set(LoadingScreen::Level);
}

fn all_shaders_compiled(resource_handles: Res<ResourceHandles>) -> bool {
    resource_handles.is_all_done()
}
