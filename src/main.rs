// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::*;
use foxtrot::GamePlugin;

fn main() {
    App::new().add_plugin(GamePlugin).run();
}
