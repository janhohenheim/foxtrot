//! The loading screen that appears when the game is starting, but still spawning the level.

use bevy::prelude::*;

use crate::{
    gameplay::{level::spawn_level, player::camera::PlayerCamera},
    screens::Screen,
    theme::prelude::*,
};

use super::LoadingScreen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(LoadingScreen::Level),
        (spawn_level, spawn_level_loading_screen),
    );
    app.add_systems(
        Update,
        advance_to_gameplay_screen.run_if(in_state(LoadingScreen::Level)),
    );
}

fn spawn_level_loading_screen(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Loading Screen"),
        StateScoped(LoadingScreen::Level),
        children![widget::label("Spawning Level...")],
    ));
}

fn advance_to_gameplay_screen(
    player_camera: Query<&PlayerCamera>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    if !player_camera.is_empty() {
        next_screen.set(Screen::Gameplay);
    }
}
