#![feature(stmt_expr_attributes)]
#![feature(let_chains)]
#![feature(fs_try_exists)]
#![feature(never_type)]
#![feature(if_let_guard)]
#![feature(once_cell)]
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
pub mod bevy_config;
#[cfg(feature = "dev")]
pub mod dev;
pub mod file_system_interaction;
pub mod ingame_menu;
pub mod level_instantiation;
pub mod menu;
pub mod movement;
#[cfg(feature = "native")]
pub mod particles;
pub mod player_control;
pub mod shader;
pub mod util;
pub mod world_interaction;

use crate::bevy_config::bevy_config_plugin;
#[cfg(feature = "dev")]
use crate::dev::DevPlugin;
use crate::file_system_interaction::FileSystemInteractionPlugin;
use crate::ingame_menu::IngameMenuPlugin;
use crate::level_instantiation::LevelInstantiationPlugin;
use crate::menu::MenuPlugin;
use crate::movement::MovementPlugin;
#[cfg(feature = "native")]
use crate::particles::ParticlePlugin;
use crate::player_control::PlayerControlPlugin;
use crate::shader::ShaderPlugin;
use crate::world_interaction::WorldInteractionPlugin;
use bevy::prelude::*;
use seldom_fn_plugin::FnPluginExt;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    /// During the loading State the LoadingPlugin will load our assets
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
/// - [`MenuPlugin`]: Handles the menu.
/// - [`MovementPlugin`]: Handles the movement of entities.
/// - [`PlayerControlPlugin`]: Handles the player's control.
/// - [`WorldInteractionPlugin`]: Handles the interaction of entities with the world.
/// - [`LevelInstantiationPlugin`]: Handles the creation of levels and objects.
/// - [`FileSystemInteractionPlugin`]: Handles the loading and saving of games.
/// - [`ShaderPlugin`]: Handles the shaders.
/// - [`DevPlugin`]: Handles the dev tools.
/// - [`IngameMenuPlugin`]: Handles the ingame menu accessed via ESC.
/// - [`ParticlePlugin`]: Handles the particle system. Since [bevy_hanabi](https://github.com/djeedai/bevy_hanabi) does not support wasm, this plugin is only available on native.
///
/// Because Foxtrot uses `seldom_fn_plugin`, these are all functions.
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        #[cfg(not(feature = "core"))]
        compile_error!("You need to compile with the core feature.");
        #[cfg(all(feature = "wasm", feature = "native"))]
        compile_error!(
            "You can only compile with the wasm or native features, not both at the same time."
        );
        #[cfg(all(feature = "native-dev", not(feature = "native")))]
        compile_error!("You can only compile with the native-dev feature if you compile with the native feature.");

        app.add_state::<GameState>()
            .fn_plugin(bevy_config_plugin)
            .fn_plugin(MenuPlugin)
            .fn_plugin(MovementPlugin)
            .fn_plugin(PlayerControlPlugin)
            .fn_plugin(WorldInteractionPlugin)
            .fn_plugin(LevelInstantiationPlugin)
            .fn_plugin(FileSystemInteractionPlugin)
            .fn_plugin(ShaderPlugin)
            .fn_plugin(IngameMenuPlugin);
        #[cfg(feature = "dev")]
        app.fn_plugin(DevPlugin);
        #[cfg(feature = "native")]
        app.fn_plugin(ParticlePlugin);
    }
}
