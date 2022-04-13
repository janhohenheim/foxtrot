// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::{App, ClearColor, Color, Msaa, WindowDescriptor};
use bevy::DefaultPlugins;
use bevy_game::GamePlugin;
#[cfg(feature = "dev")]
use bevy_inspector_egui::WorldInspectorPlugin;

fn main() {
    let mut app = App::new();
    app.insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(Color::rgb(0.4, 0.4, 0.4)))
        .insert_resource(WindowDescriptor {
            width: 800.,
            height: 600.,
            title: "Bevy game".to_string(), // ToDo
            ..Default::default()
        })
        .add_plugins(DefaultPlugins);

    #[cfg(feature = "dev")]
    app.add_plugin(WorldInspectorPlugin::new());

    app.add_plugin(GamePlugin).run();
}
