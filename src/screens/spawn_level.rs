//! The loading screen that appears when the game is starting, but still spawning the level.

use bevy::prelude::*;

use crate::{
    gameplay::{level::spawn_level, player::camera::PlayerCamera},
    screens::Screen,
    theme::{palette::SCREEN_BACKGROUND, prelude::*},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Screen::SpawnLevel),
        (spawn_level, spawn_spawn_level_screen),
    );
    app.add_systems(
        Update,
        advance_to_gameplay_screen.run_if(in_state(Screen::SpawnLevel)),
    );
}

fn spawn_spawn_level_screen(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Loading Screen"),
        StateScoped(Screen::SpawnLevel),
        BackgroundColor(SCREEN_BACKGROUND),
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
