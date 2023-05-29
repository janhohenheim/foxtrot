// disable console on windows for release builds
#![cfg_attr(not(feature = "dev"), windows_subsystem = "windows")]

use bevy::prelude::*;
use foxtrot::GamePlugin;

fn main() {
    App::new().add_plugin(GamePlugin).run();
}
