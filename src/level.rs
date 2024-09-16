//! Spawn the main level.

use bevy::{color::palettes::tailwind, prelude::*};
use blenvy::*;

use crate::{
    player::camera::spawn_player_camera,
    screens::{gameplay::GameplayState, Screen},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameplayState::SpawningLevel), spawn_level);
    app.add_systems(
        Update,
        // No need to place this in a set, as it the state transition will
        // only run next frame anyways, as `Update` is run after `StateTransition`.
        finish_spawning_level.run_if(in_state(GameplayState::SpawningLevel)),
    );
}

/// A [`Command`] to spawn the level.
/// Functions that accept only `&mut World` as their parameter implement [`Command`].
/// We use this style when a command requires no configuration.
pub fn spawn_level(world: &mut World) {
    world.spawn((
        Name::new("Level"),
        BlueprintInfo::from_path("levels/World.glb"),
        SpawnBlueprint,
        HideUntilReady,
        GameWorldTag,
        StateScoped(Screen::Gameplay),
    ));
    world.insert_resource(AmbientLight {
        color: tailwind::SKY_100.into(),
        brightness: 400.0,
    });
    spawn_player_camera(world);
}

fn finish_spawning_level(
    q_finished_level: Query<(), (With<GameWorldTag>, With<BlueprintInstanceReady>)>,
    mut next_state: ResMut<NextState<GameplayState>>,
) {
    if !q_finished_level.is_empty() {
        next_state.set(GameplayState::Playing);
    }
}
