//! The title screen that appears when the game starts.

use bevy::{prelude::*, scene::SceneInstanceReady};

use crate::{
    gameplay::level::{Level, spawn_level as spawn_level_command},
    screens::Screen,
    theme::prelude::*,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Screen::SpawnLevel),
        (spawn_level, spawn_spawn_level_screen),
    );
    app.add_systems(OnEnter(Screen::SpawnLevel), spawn_spawn_level_screen);
    app.add_observer(advance_to_gameplay_screen);
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
    trigger: Trigger<SceneInstanceReady>,
    q_level: Query<(), With<Level>>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    let level_entity = trigger.entity();
    if q_level.contains(level_entity) {
        next_screen.set(Screen::Gameplay);
    }
}
