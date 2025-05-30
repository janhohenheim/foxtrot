//! The loading screen that appears when the game is starting, but still spawning the level.

use bevy::{prelude::*, scene::SceneInstance};
use bevy_landmass::{NavMesh, coords::ThreeD};
#[cfg(feature = "hot_patch")]
use bevy_simple_subsecond_system::hot;

use crate::{
    gameplay::level::spawn_level,
    screens::Screen,
    theme::{palette::SCREEN_BACKGROUND, prelude::*},
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

#[cfg_attr(feature = "hot_patch", hot)]
fn spawn_level_loading_screen(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Loading Screen"),
        BackgroundColor(SCREEN_BACKGROUND),
        StateScoped(LoadingScreen::Level),
        children![widget::label("Spawning Level...")],
    ));
}

#[cfg_attr(feature = "hot_patch", hot)]
fn advance_to_gameplay_screen(
    mut next_screen: ResMut<NextState<Screen>>,
    scene_spawner: Res<SceneSpawner>,
    scene_instances: Query<&SceneInstance>,
    just_added_scenes: Query<(), (With<SceneRoot>, Without<SceneInstance>)>,
    just_added_meshes: Query<(), Added<Mesh3d>>,
    nav_mesh_events: EventReader<AssetEvent<NavMesh<ThreeD>>>,
) {
    if !(just_added_meshes.is_empty() && just_added_scenes.is_empty()) {
        return;
    }
    if !nav_mesh_events.is_empty() {
        return;
    }

    for scene_instance in scene_instances.iter() {
        if !scene_spawner.instance_is_ready(**scene_instance) {
            return;
        }
    }
    next_screen.set(Screen::Gameplay);
}
