// These two generate a lot of false positives for Bevy systems
#![allow(clippy::too_many_arguments, clippy::type_complexity)]
// This is not a library, so we don't need to worry about intra-doc links
#![allow(rustdoc::private_intra_doc_links)]

//! Foxtrot is split into many plugins with their own set of responsibilities.
//! This is an organizational measure and not meant to be imply that you can turn them on or off at will,
//! since the plugins are interdependent.  
//! Instead, decide for yourself which features you like and which one's you don't and simply trim the code accordingly.
//! Feel free to [file an issue](https://github.com/janhohenheim/foxtrot/issues/new) if you need help!
//! The docs are organized such that you can click through the plugins to explore the systems at play.

use bevy::prelude::*;
mod bevy_config;
#[cfg(feature = "dev")]
mod dev;
mod file_system_interaction;
mod ingame_menu;
mod level_instantiation;
mod menu;
pub(crate) mod movement;
pub(crate) mod particles;
mod player_control;
mod shader;
pub(crate) mod util;
mod world_interaction;

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
/// - [`bevy_config::plugin`]: Sets up the bevy configuration.
/// - [`menu::plugin`]: Handles the menu.
/// - [`movement::plugin`]: Handles the movement of entities.
/// - [`player_control::plugin`]: Handles the player's control.
/// - [`world_interaction::plugin`]: Handles the interaction of entities with the world.
/// - [`level_instantiation::plugin`]: Handles the creation of levels and objects.
/// - [`file_system_interaction::plugin`]: Handles the loading and saving of games.
/// - [`shader::plugin`]: Handles the shaders.
/// - [`dev::plugin`]: Handles the dev tools.
/// - [`ingame_menu::plugin`]: Handles the ingame menu accessed via ESC.
/// - [`particles::plugin`]: Handles the particle system.
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>().add_plugins((
            bevy_config::plugin,
            menu::plugin,
            movement::plugin,
            player_control::plugin,
            world_interaction::plugin,
            level_instantiation::plugin,
            file_system_interaction::plugin,
            shader::plugin,
            ingame_menu::plugin,
            particles::plugin,
            #[cfg(feature = "dev")]
            dev::plugin,
        ));
    }
}
