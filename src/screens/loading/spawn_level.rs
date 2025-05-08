//! The loading screen that appears when the game is starting, but still spawning the level.

use bevy::{audio::Volume, prelude::*};

use crate::{
    audio::Music,
    gameplay::{level::spawn_level, player::camera::PlayerCamera},
    screens::{Screen, gameplay::GameplayMusic},
    theme::prelude::*,
};

use super::LoadingScreen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(LoadingScreen::Level),
        (spawn_level, spawn_spawn_level_screen),
    );
    app.add_systems(OnEnter(LoadingScreen::Level), start_gameplay_music);
    app.add_systems(
        Update,
        advance_to_gameplay_screen.run_if(in_state(LoadingScreen::Level)),
    );
}

fn spawn_spawn_level_screen(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Loading Screen"),
        StateScoped(LoadingScreen::Level),
        children![widget::label("Spawning Level...")],
    ));
}

fn start_gameplay_music(mut commands: Commands, mut music: ResMut<GameplayMusic>) {
    music.entity = Some(
        commands
            .spawn((
                AudioPlayer(music.music.clone()),
                PlaybackSettings::LOOP.with_volume(Volume::Linear(1.5)),
                Music,
            ))
            .id(),
    );
}

fn advance_to_gameplay_screen(
    player_camera: Query<&PlayerCamera>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    if !player_camera.is_empty() {
        next_screen.set(Screen::Gameplay);
    }
}
