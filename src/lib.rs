#![feature(stmt_expr_attributes)]
#![feature(let_chains)]
#![feature(fs_try_exists)]
#![feature(never_type)]
#![feature(if_let_guard)]
#![feature(lazy_cell)]
#![feature(iter_array_chunks)]
// These two generate a lot of false positives for Bevy systems
#![allow(clippy::too_many_arguments)]
#![allow(clippy::type_complexity)]

//! Foxtrot is split into many plugins with their own set of responsibilities.
//! This is an organizational measure and not meant to be imply that you can turn them on or off at will,
//! since the plugins are interdependent.  
//! Instead, decide for yourself which features you like and which one's you don't and simply trim the code accordingly.
//! Feel free to [file an issue](https://github.com/janhohenheim/foxtrot/issues/new) if you need help!
//! The docs are organized such that you can click through the plugins to explore the systems at play.
pub(crate) mod bevy_config;
#[cfg(feature = "dev")]
pub(crate) mod dev;
pub(crate) mod file_system_interaction;
pub(crate) mod ingame_menu;
pub(crate) mod level_instantiation;
pub(crate) mod menu;
pub(crate) mod movement;
#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod particles;
pub(crate) mod player_control;
pub(crate) mod shader;
pub(crate) mod util;
pub(crate) mod world_interaction;

use crate::bevy_config::bevy_config_plugin;
#[cfg(feature = "dev")]
use crate::dev::dev_plugin;
use crate::file_system_interaction::file_system_interaction_plugin;
use crate::ingame_menu::ingame_menu_plugin;
use crate::level_instantiation::level_instantiation_plugin;
use crate::menu::menu_plugin;
use crate::movement::movement_plugin;
#[cfg(not(target_arch = "wasm32"))]
use crate::particles::particle_plugin;
use crate::player_control::player_control_plugin;
use crate::shader::shader_plugin;
use crate::world_interaction::world_interaction_plugin;
use bevy::prelude::*;
use seldom_fn_plugin::FnPluginExt;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    /// During the loading State the loading_plugin will load our assets
    #[default]
    Loading,
    /// During this State the actual game logic is executed
    Playing,
    /// Here the menu is drawn and waiting for player interaction
    Menu,
}

/// Main entrypoint for Foxtrot.
///
/// The top-level plugins are:
/// - [`bevy_config_plugin`]: Sets up the bevy configuration.
/// - [`menu_plugin`]: Handles the menu.
/// - [`movement_plugin`]: Handles the movement of entities.
/// - [`player_control_plugin`]: Handles the player's control.
/// - [`world_interaction_plugin`]: Handles the interaction of entities with the world.
/// - [`level_instantiation_plugin`]: Handles the creation of levels and objects.
/// - [`file_system_interaction_plugin`]: Handles the loading and saving of games.
/// - [`shader_plugin`]: Handles the shaders.
/// - [`dev_plugin`]: Handles the dev tools.
/// - [`ingame_menu_plugin`]: Handles the ingame menu accessed via ESC.
/// - [`particle_plugin`]: Handles the particle system. Since [bevy_hanabi](https://github.com/djeedai/bevy_hanabi) does not support wasm, this plugin is only available on native.
///
/// Because Foxtrot uses `seldom_fn_plugin`, these are all functions.
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .fn_plugin(bevy_config_plugin)
            .fn_plugin(menu_plugin)
            .fn_plugin(movement_plugin)
            .fn_plugin(player_control_plugin)
            .fn_plugin(world_interaction_plugin)
            .fn_plugin(level_instantiation_plugin)
            .fn_plugin(file_system_interaction_plugin)
            .fn_plugin(shader_plugin)
            .fn_plugin(ingame_menu_plugin);
        #[cfg(feature = "dev")]
        app.fn_plugin(dev_plugin);
        #[cfg(not(target_arch = "wasm32"))]
        app.fn_plugin(particle_plugin);
    }
}
