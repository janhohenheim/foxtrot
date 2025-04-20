//! The title screen that appears when the game starts.

use bevy::prelude::*;

use crate::{
    gameplay::{level::spawn_level as spawn_level_command, player::camera::PlayerCameraParent},
    screens::Screen,
    theme::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Screen::SpawnLevel),
        (spawn_level, spawn_spawn_level_screen),
    );
    app.add_systems(OnEnter(Screen::SpawnLevel), spawn_spawn_level_screen);
    app.add_systems(
        Update,
        advance_to_gameplay_screen.run_if(in_state(Screen::SpawnLevel)),
    );
}

fn spawn_spawn_level_screen(mut commands: Commands) {
    commands
        .ui_root()
        .insert(StateScoped(Screen::SpawnLevel))
        .with_children(|children| {
            children.label("Spawning Level...");
        });
}

fn spawn_level(mut commands: Commands) {
    commands.queue(spawn_level_command);
}

fn advance_to_gameplay_screen(
    player_camera: Query<&PlayerCameraParent>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    if !player_camera.is_empty() {
        next_screen.set(Screen::Gameplay);
    }
}
