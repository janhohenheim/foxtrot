// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy::DefaultPlugins;
use bevy_game::GamePlugin;

fn main() {
    let default_plugins = DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            width: 800.,
            height: 600.,
            title: "Bevy game".to_string(), // ToDo
            canvas: Some("#bevy".to_owned()),
            present_mode: PresentMode::AutoVsync,
            ..default()
        },
        ..default()
    });
    #[cfg(feature = "editor")]
    let default_plugins = default_plugins.set(AssetPlugin {
        watch_for_changes: true,
        ..default()
    });
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .add_plugins(default_plugins)
        .add_plugin(GamePlugin)
        .run();
}
